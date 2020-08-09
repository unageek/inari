use crate::{interval_set::*, parse::*, rel::*, visit::*};

#[derive(Clone, Debug)]
pub struct DynRelation {
    prop: Proposition,
    exprs: Vec<StaticExpr>,
    rels: Vec<StaticRel>,
    ts: Vec<TupperIntervalSet>,
    es: Vec<EvalResult>,
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
        let mut v = CollectStatic::new();
        v.visit_rel(&rel);
        let (exprs, rels) = v.exprs_rels();
        let n_ts = exprs.len() + 2;
        let n_es = rels.len();
        Self {
            prop: rel.get_proposition(),
            exprs,
            rels,
            ts: vec![TupperIntervalSet::empty(); n_ts],
            es: vec![EvalResult::default(); n_es],
        }
    }

    pub fn evaluate(&mut self, x: TupperIntervalSet, y: TupperIntervalSet) -> EvalResult {
        self.ts[0] = x;
        self.ts[1] = y;
        for i in 0..self.exprs.len() {
            self.ts[i + 2] = self.exprs[i].evaluate(&self.ts);
        }
        for i in 0..self.rels.len() {
            self.es[i] = self.rels[i].evaluate(&self.ts, &self.es);
        }
        self.es.last().unwrap().clone()
    }

    pub fn proposition(&self) -> &Proposition {
        &self.prop
    }
}
