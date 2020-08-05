use crate::ast::*;
use std::{
    collections::{HashMap, HashSet},
    marker::Sized,
};

pub trait Visit<'a>
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

fn traverse_expr<'a, V: Visit<'a>>(v: &mut V, expr: &'a Expr) {
    use ExprKind::*;
    match &expr.kind {
        Unary(_, x) => v.visit_expr(x),
        Binary(_, x, y) => {
            v.visit_expr(x);
            v.visit_expr(y);
        }
        Pown(x, _) => v.visit_expr(x),
        _ => (),
    };
}

fn traverse_rel<'a, V: Visit<'a>>(v: &mut V, rel: &'a Rel) {
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

pub trait VisitMut
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

fn traverse_expr_mut<V: VisitMut>(v: &mut V, expr: &mut Expr) {
    use ExprKind::*;
    match &mut expr.kind {
        Unary(_, x) => v.visit_expr_mut(x),
        Binary(_, x, y) => {
            v.visit_expr_mut(x);
            v.visit_expr_mut(y);
        }
        Pown(x, _) => v.visit_expr_mut(x),
        _ => (),
    };
}

fn traverse_rel_mut<V: VisitMut>(v: &mut V, rel: &mut Rel) {
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

type SiteMap = HashMap<ExprId, Option<u8>>;

pub struct Transform;

impl VisitMut for Transform {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        use {BinaryOp::*, ExprKind::*, UnaryOp::*};
        traverse_expr_mut(self, expr);

        match &mut expr.kind {
            Binary(Div, x, y) => {
                match (&x.kind, &y.kind) {
                    // (Div (Sin z) y) => (SinOverX y) if z == y
                    (Unary(Sin, z), _) if z == y => {
                        *expr = Expr::new(Unary(SinOverX, std::mem::take(y)));
                    }
                    // (Div x (Sin z)) => (Recip (SinOverX x)) if z == x
                    (_, Unary(Sin, z)) if z == x => {
                        *expr = Expr::new(Unary(
                            Recip,
                            Box::new(Expr::new(Unary(SinOverX, std::mem::take(x)))),
                        ));
                    }
                    _ => (),
                };
            }
            Pown(x, y) => match y {
                -1 => {
                    *expr = Expr::new(Unary(Recip, std::mem::take(x)));
                }
                // Do not transform x^0 to 1.0 as that could discard the decoration.
                1 => {
                    *expr = *std::mem::take(x);
                }
                2 => {
                    *expr = Expr::new(Unary(Sqr, std::mem::take(x)));
                }
                _ => (),
            },
            _ => (),
        }
    }
}

pub struct FoldConstant;

// Only fold constants which evaluate to an empty or a single interval
// because sites are not assigned and branch cut tracking cannot be done
// at this moment.
impl VisitMut for FoldConstant {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        use ExprKind::*;
        traverse_expr_mut(self, expr);

        match &mut expr.kind {
            Unary(_, x) => {
                if let Constant(_) = &x.kind {
                    let value = expr.evaluate_constant();
                    if value.len() <= 1 {
                        *expr = Expr::new(Constant(value));
                    }
                }
            }
            Binary(_, x, y) => {
                if let (Constant(_), Constant(_)) = (&x.kind, &y.kind) {
                    let value = expr.evaluate_constant();
                    if value.len() <= 1 {
                        *expr = Expr::new(Constant(value));
                    }
                }
            }
            Pown(x, _) => {
                if let Constant(_) = &x.kind {
                    let value = expr.evaluate_constant();
                    if value.len() <= 1 {
                        *expr = Expr::new(Constant(value));
                    }
                }
            }
            _ => (),
        }
    }
}

pub struct AssignId<'a> {
    next_id: ExprId,
    next_site: u8,
    site_map: SiteMap,
    visited_nodes: HashSet<&'a Expr>,
}

impl<'a> AssignId<'a> {
    pub fn new() -> Self {
        AssignId {
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
        use {BinaryOp::*, ExprKind::*, UnaryOp::*};
        matches!(kind,
            Unary(Ceil, _)
            | Unary(Floor, _)
            | Unary(Recip, _)
            | Unary(Sign, _)
            | Unary(Tan, _)
            | Binary(Div, _, _)
            | Binary(Atan2, _, _)
            | Binary(Mod, _, _)
            | Pown(_, _))
    }
}

impl<'a> Visit<'a> for AssignId<'a> {
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

impl<'a> Visit<'a> for AssignSite {
    fn visit_expr(&mut self, expr: &'a Expr) {
        traverse_expr(self, expr);

        if let Some(site) = self.site_map.get(&expr.id.get()) {
            expr.site.set(*site);
        }
    }
}

// Collects nodes (except the ones for X and Y), sorted topologically.
pub struct CollectExprsForEvaluation {
    exprs: Vec<Expr>,
    next_id: ExprId,
}

impl CollectExprsForEvaluation {
    pub fn new() -> Self {
        CollectExprsForEvaluation {
            exprs: Vec::new(),
            next_id: 2, // 0 for x, 1 for y
        }
    }

    pub fn exprs(self) -> Vec<Expr> {
        self.exprs
    }
}

impl<'a> Visit<'a> for CollectExprsForEvaluation {
    fn visit_expr(&mut self, expr: &'a Expr) {
        traverse_expr(self, expr);

        if expr.id.get() == self.next_id {
            self.exprs.push(expr.clone_for_evaluation());
            self.next_id += 1;
        }
    }
}
