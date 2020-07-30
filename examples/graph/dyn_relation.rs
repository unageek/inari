use crate::{ast::*, interval_set::*, parse::*, visitor::*};

#[derive(Clone, Debug)]
pub struct DynRelation {
    rel: Rel,
    nodes: Vec<Expr>,
    vs: ValueStore,
}

impl DynRelation {
    pub fn new(relation: &str) -> Self {
        let mut rel = parse(relation).unwrap();
        let mut v = AssignNodeId::new();
        v.visit_rel_mut(&mut rel);
        let mut v = AssignSite::new(v.site_map());
        v.visit_rel_mut(&mut rel);
        let mut v = CollectNodes::new();
        v.visit_rel(&rel);
        let nodes = v.nodes();
        let n = nodes.len() + 2;
        Self {
            rel,
            nodes,
            vs: vec![TupperIntervalSet::new(); n],
        }
    }

    pub fn evaluate(&mut self, x: TupperIntervalSet, y: TupperIntervalSet) -> EvaluationResult {
        self.vs[0] = x;
        self.vs[1] = y;
        for i in 0..self.nodes.len() {
            self.vs[i + 2] = self.nodes[i].evaluate(&self.vs);
        }
        self.rel.evaluate(&self.vs)
    }
}
