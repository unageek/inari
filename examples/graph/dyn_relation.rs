use crate::{ast::*, interval_set::*, parse::*, visit::*};

#[derive(Clone, Debug)]
pub struct DynRelation {
    rel: Rel,
    exprs: Vec<Expr>,
    vs: ValueStore,
}

impl DynRelation {
    pub fn new(relation: &str) -> Self {
        let mut rel = parse(relation).unwrap();
        Transform.visit_rel_mut(&mut rel);
        FoldConstant.visit_rel_mut(&mut rel);
        let mut v = AssignId::new();
        v.visit_rel(&rel);
        let mut v = AssignSite::new(v.site_map());
        v.visit_rel(&rel);
        let mut v = CollectExprsForEvaluation::new();
        v.visit_rel(&rel);
        let exprs = v.exprs();
        let n = exprs.len() + 2;
        Self {
            rel,
            exprs,
            vs: vec![TupperIntervalSet::empty(); n],
        }
    }

    pub fn evaluate(&mut self, x: TupperIntervalSet, y: TupperIntervalSet) -> EvaluationResult {
        self.vs[0] = x;
        self.vs[1] = y;
        for i in 0..self.exprs.len() {
            self.vs[i + 2] = self.exprs[i].evaluate(&self.vs);
        }
        self.rel.evaluate(&self.vs)
    }
}
