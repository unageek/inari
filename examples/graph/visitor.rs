use crate::ast::*;
use std::{
    collections::{HashMap, HashSet},
    marker::Sized,
};

// TODO:
// - Constant folding
// - Cloning every nodes as done in some visitors is stupid

pub trait Visitor
where
    Self: Sized,
{
    fn visit_expr(&mut self, expr: &Expr) {
        traverse_expr(self, expr);
    }

    fn visit_rel(&mut self, rel: &Rel) {
        traverse_rel(self, rel)
    }
}

fn traverse_expr<V: Visitor>(v: &mut V, expr: &Expr) {
    use ExprKind::*;
    match &expr.kind {
        Unary(_, x) => v.visit_expr(x),
        Binary(_, x, y) => {
            v.visit_expr(x);
            v.visit_expr(y);
        }
        _ => (),
    };
}

fn traverse_rel<V: Visitor>(v: &mut V, rel: &Rel) {
    use RelKind::*;
    match &rel.kind {
        Equality(_, x, y) => {
            v.visit_expr(x);
            v.visit_expr(y);
        }
        Binary(_, x, y) => {
            v.visit_rel(x);
            v.visit_rel(y);
        }
    };
}

pub trait VisitorMut
where
    Self: Sized,
{
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        traverse_expr_mut(self, expr);
    }

    fn visit_rel_mut(&mut self, rel: &mut Rel) {
        traverse_rel_mut(self, rel);
    }
}

fn traverse_expr_mut<V: VisitorMut>(v: &mut V, expr: &mut Expr) {
    use ExprKind::*;
    match &mut expr.kind {
        Unary(_, x) => v.visit_expr_mut(x),
        Binary(_, x, y) => {
            v.visit_expr_mut(x);
            v.visit_expr_mut(y);
        }
        _ => (),
    };
}

fn traverse_rel_mut<V: VisitorMut>(v: &mut V, rel: &mut Rel) {
    use RelKind::*;
    match &mut rel.kind {
        Equality(_, x, y) => {
            v.visit_expr_mut(x);
            v.visit_expr_mut(y);
        }
        Binary(_, x, y) => {
            v.visit_rel_mut(x);
            v.visit_rel_mut(y);
        }
    };
}

type SiteMap = HashMap<NodeId, Option<u8>>;

pub struct Transform;

impl VisitorMut for Transform {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        traverse_expr_mut(self, expr);

        if let ExprKind::Binary(BinaryOp::Div, x, y) = &mut expr.kind {
            match (&x.kind, &y.kind) {
                (ExprKind::Unary(UnaryOp::Sin, z), _) if z == y => {
                    // (Div (Sin z) y) => (SinOverX y)
                    *expr = Expr::new(ExprKind::Unary(UnaryOp::SinOverX, std::mem::take(y)));
                }
                (_, ExprKind::Unary(UnaryOp::Sin, z)) if z == x => {
                    // (Div x (Sin z)) => (Recip (SinOverX x))
                    *expr = Expr::new(ExprKind::Unary(
                        UnaryOp::Recip,
                        Box::new(Expr::new(ExprKind::Unary(
                            UnaryOp::SinOverX,
                            std::mem::take(x),
                        ))),
                    ));
                }
                _ => (),
            };
        }
    }
}

pub struct AssignNodeId {
    next_id: NodeId,
    next_site: u8,
    site_map: SiteMap,
    visited_nodes: HashSet<Expr>,
}

impl AssignNodeId {
    pub fn new() -> Self {
        Self {
            next_id: 2, // 0 for x, 1 for y
            next_site: 0,
            site_map: HashMap::new(),
            visited_nodes: HashSet::new(),
        }
    }

    pub fn site_map(self) -> SiteMap {
        self.site_map
    }

    fn expr_can_perform_cut(kind: &ExprKind) -> bool {
        use BinaryOp::*;
        use ExprKind::*;
        use UnaryOp::*;
        matches!(kind,
            Unary(Ceil, _)
            | Unary(Floor, _)
            | Unary(Recip, _)
            | Unary(Sign, _)
            | Unary(Tan, _)
            | Binary(Div, _, _)
            | Binary(Atan2, _, _))
    }
}

impl VisitorMut for AssignNodeId {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        traverse_expr_mut(self, expr);

        match self.visited_nodes.get(expr) {
            Some(visited) => {
                expr.id = visited.id;

                if let Some(site) = self.site_map.get_mut(&expr.id) {
                    if site.is_none() && self.next_site <= 31 {
                        *site = Some(self.next_site as u8);
                        self.next_site += 1;
                    }
                }
            }
            _ => {
                expr.id = match &expr.kind {
                    ExprKind::X => 0,
                    ExprKind::Y => 1,
                    _ => {
                        let i = self.next_id;
                        self.next_id += 1;
                        i
                    }
                };

                if Self::expr_can_perform_cut(&expr.kind) {
                    self.site_map.insert(expr.id, None);
                }

                self.visited_nodes.insert(expr.clone());
            }
        }
    }
}

pub struct AssignSite {
    site_map: SiteMap,
}

impl AssignSite {
    pub fn new(site_map: SiteMap) -> Self {
        Self { site_map }
    }
}

impl VisitorMut for AssignSite {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        traverse_expr_mut(self, expr);

        if let Some(site) = self.site_map.get(&expr.id) {
            expr.site = *site;
        }
    }
}

// Collects nodes (except the ones for x and y) sorted topologically.
pub struct CollectNodes {
    nodes: Vec<Expr>,
    next_node_id: NodeId,
}

impl CollectNodes {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            next_node_id: 2, // 0 for x, 1 for y
        }
    }

    pub fn nodes(self) -> Vec<Expr> {
        self.nodes
    }
}

impl Visitor for CollectNodes {
    fn visit_expr(&mut self, expr: &Expr) {
        traverse_expr(self, expr);

        if expr.id == self.next_node_id {
            self.nodes.push(expr.clone());
            self.next_node_id += 1;
        }
    }
}
