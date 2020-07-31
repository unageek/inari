use crate::interval_set::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnaryOp {
    Neg,
    // Builtin functions
    Abs,
    Acos,
    Acosh,
    Asin,
    Asinh,
    Atan,
    Atanh,
    Ceil,
    Cos,
    Cosh,
    Exp,
    Exp10,
    Exp2,
    Floor,
    Log,
    Log10,
    Log2,
    Sign,
    Sin,
    Sinh,
    Sqr,
    Sqrt,
    Tan,
    Tanh,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BinaryOp {
    Add,
    Div,
    Mul,
    Sub,
    // Builtin functions
    Atan2,
    Max,
    Min,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EqualityOp {
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RelBinaryOp {
    And,
    Or,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ExprKind {
    Constant(TupperIntervalSet),
    X,
    Y,
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RelKind {
    Equality(EqualityOp, Box<Expr>, Box<Expr>),
    Binary(RelBinaryOp, Box<Rel>, Box<Rel>),
}

pub type NodeId = u32;

#[derive(Clone, Debug)]
pub struct Expr {
    pub id: NodeId,
    pub site: Option<u8>,
    pub kind: ExprKind,
}

impl PartialEq for Expr {
    fn eq(&self, rhs: &Self) -> bool {
        self.kind == rhs.kind
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct Rel {
    pub kind: RelKind,
}

impl PartialEq for Rel {
    fn eq(&self, rhs: &Self) -> bool {
        self.kind == rhs.kind
    }
}

impl Eq for Rel {}

impl Hash for Rel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}

pub type ValueStore = Vec<TupperIntervalSet>;
pub type ValueStoreSlice = [TupperIntervalSet];

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Self {
            id: 0,
            site: None,
            kind,
        }
    }

    pub fn evaluate(&self, vs: &ValueStoreSlice) -> TupperIntervalSet {
        use {BinaryOp::*, ExprKind::*, UnaryOp::*};
        match &self.kind {
            Constant(x) => x.clone(),
            X => vs[0].clone(),
            Y => vs[1].clone(),
            Unary(Neg, x) => -x.value(vs),
            Unary(Abs, x) => x.value(vs).abs(),
            Unary(Acos, x) => x.value(vs).acos(),
            Unary(Acosh, x) => x.value(vs).acosh(),
            Unary(Asin, x) => x.value(vs).asin(),
            Unary(Asinh, x) => x.value(vs).asinh(),
            Unary(Atan, x) => x.value(vs).atan(),
            Unary(Atanh, x) => x.value(vs).atanh(),
            Unary(Ceil, x) => x.value(vs).ceil(self.site),
            Unary(Cos, x) => x.value(vs).cos(),
            Unary(Cosh, x) => x.value(vs).cosh(),
            Unary(Exp, x) => x.value(vs).exp(),
            Unary(Exp10, x) => x.value(vs).exp10(),
            Unary(Exp2, x) => x.value(vs).exp2(),
            Unary(Floor, x) => x.value(vs).floor(self.site),
            Unary(Log, x) => x.value(vs).log(),
            Unary(Log10, x) => x.value(vs).log10(),
            Unary(Log2, x) => x.value(vs).log2(),
            Unary(Sign, x) => x.value(vs).sign(self.site),
            Unary(Sin, x) => x.value(vs).sin(),
            Unary(Sinh, x) => x.value(vs).sinh(),
            Unary(Sqr, x) => x.value(vs).sqr(),
            Unary(Sqrt, x) => x.value(vs).sqrt(),
            Unary(Tan, x) => x.value(vs).tan(self.site),
            Unary(Tanh, x) => x.value(vs).tanh(),
            Binary(Add, x, y) => x.value(vs) + y.value(vs),
            Binary(Sub, x, y) => x.value(vs) - y.value(vs),
            Binary(Mul, x, y) => x.value(vs) * y.value(vs),
            Binary(Div, x, y) => x.value(vs).div(y.value(vs), self.site),
            Binary(Atan2, x, y) => x.value(vs).atan2(y.value(vs), self.site),
            Binary(Max, x, y) => x.value(vs).max(y.value(vs)),
            Binary(Min, x, y) => x.value(vs).min(y.value(vs)),
        }
    }

    fn value<'a>(&self, vs: &'a ValueStoreSlice) -> &'a TupperIntervalSet {
        &vs[self.id as usize]
    }
}

impl Rel {
    pub fn new(kind: RelKind) -> Self {
        Self { kind }
    }

    pub fn evaluate(&self, vs: &ValueStoreSlice) -> EvaluationResult {
        use {EqualityOp::*, RelBinaryOp::*, RelKind::*};
        match &self.kind {
            Equality(Eq, x, y) => x.value(vs).eq(&y.value(vs)),
            Equality(Ge, x, y) => x.value(vs).ge(&y.value(vs)),
            Equality(Gt, x, y) => x.value(vs).gt(&y.value(vs)),
            Equality(Le, x, y) => x.value(vs).le(&y.value(vs)),
            Equality(Lt, x, y) => x.value(vs).lt(&y.value(vs)),
            Binary(And, x, y) => x.evaluate(vs) & y.evaluate(vs),
            Binary(Or, x, y) => x.evaluate(vs) | y.evaluate(vs),
        }
    }
}
