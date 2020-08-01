use crate::ast::*;
use std::{
    collections::{HashMap, HashSet},
    marker::Sized,
};

// TODO:
// - Constant folding

pub trait Visitor<'a>
where
    Self: Sized,
{
    fn visit_expr(&mut self, expr: &'a Expr) {
        traverse_expr(self, expr);
    }

    fn visit_rel(&mut self, rel: &'a Rel) {
        traverse_rel(self, rel)
    }
}

fn traverse_expr<'a, V: Visitor<'a>>(v: &mut V, expr: &'a Expr) {
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

fn traverse_rel<'a, V: Visitor<'a>>(v: &mut V, rel: &'a Rel) {
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

pub struct AssignNodeId<'a> {
    next_id: NodeId,
    next_site: u8,
    site_map: SiteMap,
    visited_nodes: HashSet<&'a Expr>,
}

impl<'a> AssignNodeId<'a> {
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

impl<'a> Visitor<'a> for AssignNodeId<'a> {
    fn visit_expr(&mut self, expr: &'a Expr) {
        traverse_expr(self, expr);

        match self.visited_nodes.get(expr) {
            Some(visited) => {
                let id = visited.id.get();
                expr.id.set(id);

                if let Some(site) = self.site_map.get_mut(&id) {
                    if site.is_none() && self.next_site <= 31 {
                        *site = Some(self.next_site as u8);
                        self.next_site += 1;
                    }
                }
            }
            _ => {
                let id = match &expr.kind {
                    ExprKind::X => 0,
                    ExprKind::Y => 1,
                    _ => {
                        let i = self.next_id;
                        self.next_id += 1;
                        i
                    }
                };
                expr.id.set(id);

                if Self::expr_can_perform_cut(&expr.kind) {
                    self.site_map.insert(id, None);
                }

                self.visited_nodes.insert(expr);
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

impl<'a> Visitor<'a> for AssignSite {
    fn visit_expr(&mut self, expr: &'a Expr) {
        traverse_expr(self, expr);

        if let Some(site) = self.site_map.get(&expr.id.get()) {
            expr.site.set(*site);
        }
    }
}

// Collects nodes for evaluation (except the ones for x and y), sorted topologically.
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

impl<'a> Visitor<'a> for CollectNodes {
    fn visit_expr(&mut self, expr: &'a Expr) {
        traverse_expr(self, expr);

        if expr.id.get() == self.next_node_id {
            self.nodes.push(expr.clone_shallow());
            self.next_node_id += 1;
        }
    }
}
