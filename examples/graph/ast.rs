use crate::interval_set::*;
use std::{
    cell::Cell,
    hash::{Hash, Hasher},
};

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
    Recip,
    Sign,
    Sin,
    SinOverX,
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
    Mod,
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
    Pown(Box<Expr>, i32),
    Never,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RelKind {
    Equality(EqualityOp, Box<Expr>, Box<Expr>),
    Binary(RelBinaryOp, Box<Rel>, Box<Rel>),
}

pub type ExprId = u32;
const NO_EXPR_ID: ExprId = ExprId::MAX;

#[derive(Clone, Debug)]
pub struct Expr {
    pub id: Cell<ExprId>,
    pub site: Cell<Option<u8>>,
    pub kind: ExprKind,
}

impl Default for Expr {
    fn default() -> Self {
        Self {
            id: Cell::new(NO_EXPR_ID),
            site: Cell::new(None),
            kind: ExprKind::Never,
        }
    }
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
            id: Cell::new(NO_EXPR_ID),
            site: Cell::new(None),
            kind,
        }
    }

    // Clones self with minimum information required for evaluation.
    pub fn clone_for_evaluation(&self) -> Self {
        use ExprKind::*;
        let kind = match &self.kind {
            Unary(op, x) => Unary(
                *op,
                box Expr {
                    id: x.id.clone(),
                    site: Cell::new(None),
                    kind: ExprKind::Never,
                },
            ),
            Binary(op, x, y) => Binary(
                *op,
                box Expr {
                    id: x.id.clone(),
                    site: Cell::new(None),
                    kind: ExprKind::Never,
                },
                box Expr {
                    id: y.id.clone(),
                    site: Cell::new(None),
                    kind: ExprKind::Never,
                },
            ),
            Pown(x, y) => Pown(
                box Expr {
                    id: x.id.clone(),
                    site: Cell::new(None),
                    kind: ExprKind::Never,
                },
                *y,
            ),
            x => x.clone(),
        };
        Self {
            id: Cell::new(NO_EXPR_ID),
            site: self.site.clone(),
            kind,
        }
    }

    pub fn evaluate_constant(&self) -> TupperIntervalSet {
        use {BinaryOp::*, ExprKind::*, UnaryOp::*};
        match &self.kind {
            Constant(x) => x.clone(),
            Unary(Neg, x) => -&x.evaluate_constant(),
            Unary(Abs, x) => x.evaluate_constant().abs(),
            Unary(Acos, x) => x.evaluate_constant().acos(),
            Unary(Acosh, x) => x.evaluate_constant().acosh(),
            Unary(Asin, x) => x.evaluate_constant().asin(),
            Unary(Asinh, x) => x.evaluate_constant().asinh(),
            Unary(Atan, x) => x.evaluate_constant().atan(),
            Unary(Atanh, x) => x.evaluate_constant().atanh(),
            Unary(Ceil, x) => x.evaluate_constant().ceil(None),
            Unary(Cos, x) => x.evaluate_constant().cos(),
            Unary(Cosh, x) => x.evaluate_constant().cosh(),
            Unary(Exp, x) => x.evaluate_constant().exp(),
            Unary(Exp10, x) => x.evaluate_constant().exp10(),
            Unary(Exp2, x) => x.evaluate_constant().exp2(),
            Unary(Floor, x) => x.evaluate_constant().floor(None),
            Unary(Log, x) => x.evaluate_constant().log(),
            Unary(Log10, x) => x.evaluate_constant().log10(),
            Unary(Log2, x) => x.evaluate_constant().log2(),
            Unary(Recip, x) => x.evaluate_constant().recip(None),
            Unary(Sign, x) => x.evaluate_constant().sign(None),
            Unary(Sin, x) => x.evaluate_constant().sin(),
            Unary(SinOverX, x) => x.evaluate_constant().sin_over_x(),
            Unary(Sinh, x) => x.evaluate_constant().sinh(),
            Unary(Sqr, x) => x.evaluate_constant().sqr(),
            Unary(Sqrt, x) => x.evaluate_constant().sqrt(),
            Unary(Tan, x) => x.evaluate_constant().tan(None),
            Unary(Tanh, x) => x.evaluate_constant().tanh(),
            Binary(Add, x, y) => &x.evaluate_constant() + &y.evaluate_constant(),
            Binary(Sub, x, y) => &x.evaluate_constant() - &y.evaluate_constant(),
            Binary(Mul, x, y) => &x.evaluate_constant() * &y.evaluate_constant(),
            Binary(Div, x, y) => x.evaluate_constant().div(&y.evaluate_constant(), None),
            Binary(Atan2, x, y) => x.evaluate_constant().atan2(&y.evaluate_constant(), None),
            Binary(Max, x, y) => x.evaluate_constant().max(&y.evaluate_constant()),
            Binary(Min, x, y) => x.evaluate_constant().min(&y.evaluate_constant()),
            Binary(Mod, x, y) => x
                .evaluate_constant()
                .rem_euclid(&y.evaluate_constant(), None),
            Pown(x, y) => x.evaluate_constant().pown(*y, None),
            X | Y | Never => panic!("The expression is not a constant."),
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
            Unary(Ceil, x) => x.value(vs).ceil(self.site.get()),
            Unary(Cos, x) => x.value(vs).cos(),
            Unary(Cosh, x) => x.value(vs).cosh(),
            Unary(Exp, x) => x.value(vs).exp(),
            Unary(Exp10, x) => x.value(vs).exp10(),
            Unary(Exp2, x) => x.value(vs).exp2(),
            Unary(Floor, x) => x.value(vs).floor(self.site.get()),
            Unary(Log, x) => x.value(vs).log(),
            Unary(Log10, x) => x.value(vs).log10(),
            Unary(Log2, x) => x.value(vs).log2(),
            Unary(Recip, x) => x.value(vs).recip(self.site.get()),
            Unary(Sign, x) => x.value(vs).sign(self.site.get()),
            Unary(Sin, x) => x.value(vs).sin(),
            Unary(SinOverX, x) => x.value(vs).sin_over_x(),
            Unary(Sinh, x) => x.value(vs).sinh(),
            Unary(Sqr, x) => x.value(vs).sqr(),
            Unary(Sqrt, x) => x.value(vs).sqrt(),
            Unary(Tan, x) => x.value(vs).tan(self.site.get()),
            Unary(Tanh, x) => x.value(vs).tanh(),
            Binary(Add, x, y) => x.value(vs) + y.value(vs),
            Binary(Sub, x, y) => x.value(vs) - y.value(vs),
            Binary(Mul, x, y) => x.value(vs) * y.value(vs),
            Binary(Div, x, y) => x.value(vs).div(y.value(vs), self.site.get()),
            Binary(Atan2, x, y) => x.value(vs).atan2(y.value(vs), self.site.get()),
            Binary(Max, x, y) => x.value(vs).max(y.value(vs)),
            Binary(Min, x, y) => x.value(vs).min(y.value(vs)),
            Binary(Mod, x, y) => x.value(vs).rem_euclid(y.value(vs), self.site.get()),
            Pown(x, y) => x.value(vs).pown(*y, self.site.get()),
            Never => panic!(),
        }
    }

    fn value<'a>(&self, vs: &'a ValueStoreSlice) -> &'a TupperIntervalSet {
        &vs[self.id.get() as usize]
    }
}

impl Rel {
    pub fn new(kind: RelKind) -> Self {
        Self { kind }
    }

    pub fn evaluate(&self, vs: &ValueStoreSlice) -> EvalResult {
        use {EqualityOp::*, RelBinaryOp::*, RelKind::*};
        match &self.kind {
            Equality(Eq, x, y) => x.value(vs).eq(&y.value(vs)),
            Equality(Ge, x, y) => x.value(vs).ge(&y.value(vs)),
            Equality(Gt, x, y) => x.value(vs).gt(&y.value(vs)),
            Equality(Le, x, y) => x.value(vs).le(&y.value(vs)),
            Equality(Lt, x, y) => x.value(vs).lt(&y.value(vs)),
            Binary(And, x, y) => EvalResult([x.evaluate(vs).0, y.evaluate(vs).0].concat()),
            Binary(Or, x, y) => EvalResult([x.evaluate(vs).0, y.evaluate(vs).0].concat()),
        }
    }

    pub fn get_proposition(&self) -> Proposition {
        use {RelBinaryOp::*, RelKind::*};
        match &self.kind {
            Equality(_, _, _) => Proposition {
                kind: PropositionKind::Atomic,
                size: 1,
            },
            Binary(And, x, y) => {
                let px = x.get_proposition();
                let py = y.get_proposition();
                let size = px.size + py.size;
                Proposition {
                    kind: PropositionKind::And(box (px, py)),
                    size,
                }
            }
            Binary(Or, x, y) => {
                let px = x.get_proposition();
                let py = y.get_proposition();
                let size = px.size + py.size;
                Proposition {
                    kind: PropositionKind::Or(box (px, py)),
                    size,
                }
            }
        }
    }
}
