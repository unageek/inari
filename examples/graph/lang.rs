use crate::interval_set::*;
use inari::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit0, digit1, space0},
    combinator::{cut, map, opt, recognize, value},
    multi::fold_many0,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    marker::Sized,
};

// TODO:
// - Constant folding
// - Some transformation (sin(x)/x -> sin_over_x(x), etc.)
// - Cloning every nodes as done in some visitors is stupid
// - Error handling

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum UnaryOp {
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
    Sqrt,
    Tan,
    Tanh,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum BinaryOp {
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
enum EqualityOp {
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum RelBinaryOp {
    And,
    Or,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum ExprKind {
    Constant(TupperIntervalSet),
    X,
    Y,
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum RelKind {
    Equality(EqualityOp, Box<Expr>, Box<Expr>),
    Binary(RelBinaryOp, Box<Rel>, Box<Rel>),
}

type NodeId = u32;

#[derive(Clone, Debug)]
pub struct Expr {
    id: NodeId,
    site: Option<u8>,
    kind: ExprKind,
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
    kind: RelKind,
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

type ValueStore = Vec<TupperIntervalSet>;

impl Expr {
    fn new(kind: ExprKind) -> Self {
        Self {
            id: 0,
            site: None,
            kind,
        }
    }

    fn accept<V: Visitor>(&self, visitor: &mut V) {
        use ExprKind::*;
        match &self.kind {
            Unary(_, x) => x.accept(visitor),
            Binary(_, x, y) => {
                x.accept(visitor);
                y.accept(visitor);
            }
            _ => (),
        };
        visitor.visit_expr(self);
    }

    fn accept_mut<V: VisitorMut>(&mut self, visitor: &mut V) {
        use ExprKind::*;
        match &mut self.kind {
            Unary(_, x) => x.accept_mut(visitor),
            Binary(_, x, y) => {
                x.accept_mut(visitor);
                y.accept_mut(visitor);
            }
            _ => (),
        };
        visitor.visit_expr_mut(self);
    }

    fn evaluate(&self, vs: &ValueStore) -> TupperIntervalSet {
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

    fn value<'a>(&self, vs: &'a ValueStore) -> &'a TupperIntervalSet {
        &vs[self.id as usize]
    }
}

impl Rel {
    fn new(kind: RelKind) -> Self {
        Self { kind }
    }

    fn accept<V: Visitor>(&self, visitor: &mut V) {
        use RelKind::*;
        match &self.kind {
            Equality(_, x, y) => {
                x.accept(visitor);
                y.accept(visitor);
            }
            Binary(_, x, y) => {
                x.accept(visitor);
                y.accept(visitor);
            }
        };
        visitor.visit_rel(self);
    }

    fn accept_mut<V: VisitorMut>(&mut self, visitor: &mut V) {
        use RelKind::*;
        match &mut self.kind {
            Equality(_, x, y) => {
                x.accept_mut(visitor);
                y.accept_mut(visitor);
            }
            Binary(_, x, y) => {
                x.accept_mut(visitor);
                y.accept_mut(visitor);
            }
        };
        visitor.visit_rel_mut(self);
    }

    fn evaluate(&self, vs: &ValueStore) -> EvaluationResult {
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

fn decimal_literal(i: &str) -> IResult<&str, &str> {
    alt((
        recognize(pair(digit1, opt(pair(char('.'), digit0)))),
        recognize(pair(char('.'), digit1)),
    ))(i)
}

fn primary_expr(i: &str) -> IResult<&str, Expr> {
    alt((
        map(decimal_literal, |s| {
            let s = ["[", s, ",", s, "]"].concat();
            let x = TupperIntervalSet::from(dec_interval!(&s).unwrap());
            Expr::new(ExprKind::Constant(x))
        }),
        map(tag("pi"), |_| {
            let x = TupperIntervalSet::from(DecoratedInterval::PI);
            Expr::new(ExprKind::Constant(x))
        }),
        map(char('e'), |_| {
            let x = TupperIntervalSet::from(DecoratedInterval::E);
            Expr::new(ExprKind::Constant(x))
        }),
        value(Expr::new(ExprKind::X), char('x')),
        value(Expr::new(ExprKind::Y), char('y')),
        delimited(
            terminated(char('('), space0),
            additive_expr,
            preceded(space0, cut(char(')'))),
        ),
    ))(i)
}

fn fn1(i: &str) -> IResult<&str, UnaryOp> {
    // alt is limited to 21 choices.
    alt((
        value(UnaryOp::Acosh, tag("acosh")),
        value(UnaryOp::Asinh, tag("asinh")),
        value(UnaryOp::Atanh, tag("atanh")),
        value(UnaryOp::Exp10, tag("exp10")),
        value(UnaryOp::Floor, tag("floor")),
        value(UnaryOp::Log10, tag("log10")),
        value(UnaryOp::Acos, tag("acos")),
        value(UnaryOp::Asin, tag("asin")),
        value(UnaryOp::Atan, tag("atan")),
        value(UnaryOp::Ceil, tag("ceil")),
        value(UnaryOp::Cosh, tag("cosh")),
        value(UnaryOp::Exp2, tag("exp2")),
        value(UnaryOp::Log2, tag("log2")),
        value(UnaryOp::Sign, tag("sign")),
        value(UnaryOp::Sinh, tag("sinh")),
        value(UnaryOp::Sqrt, tag("sqrt")),
        value(UnaryOp::Tanh, tag("tanh")),
        alt((
            value(UnaryOp::Abs, tag("abs")),
            value(UnaryOp::Cos, tag("cos")),
            value(UnaryOp::Exp, tag("exp")),
            value(UnaryOp::Log, tag("log")),
            value(UnaryOp::Sin, tag("sin")),
            value(UnaryOp::Tan, tag("tan")),
        )),
    ))(i)
}

fn fn2(i: &str) -> IResult<&str, BinaryOp> {
    alt((
        value(BinaryOp::Atan2, tag("atan2")),
        value(BinaryOp::Max, tag("max")),
        value(BinaryOp::Min, tag("min")),
    ))(i)
}

fn postfix_expr(i: &str) -> IResult<&str, Expr> {
    alt((
        map(
            pair(
                fn1,
                delimited(
                    terminated(char('('), space0),
                    additive_expr,
                    preceded(space0, cut(char(')'))),
                ),
            ),
            move |(f, x)| Expr::new(ExprKind::Unary(f, Box::new(x))),
        ),
        map(
            pair(
                fn2,
                delimited(
                    terminated(char('('), space0),
                    separated_pair(
                        additive_expr,
                        delimited(space0, char(','), space0),
                        additive_expr,
                    ),
                    preceded(space0, cut(char(')'))),
                ),
            ),
            move |(f, (x, y))| Expr::new(ExprKind::Binary(f, Box::new(x), Box::new(y))),
        ),
        primary_expr,
    ))(i)
}

fn unary_expr(i: &str) -> IResult<&str, Expr> {
    alt((
        map(
            separated_pair(value(UnaryOp::Neg, char('-')), space0, unary_expr),
            |(op, x)| Expr::new(ExprKind::Unary(op, Box::new(x))),
        ),
        postfix_expr,
    ))(i)
}

fn multiplicative_expr(i: &str) -> IResult<&str, Expr> {
    let (i, x) = unary_expr(i)?;

    fold_many0(
        pair(
            delimited(
                space0,
                alt((
                    value(BinaryOp::Mul, char('*')),
                    value(BinaryOp::Div, char('/')),
                )),
                space0,
            ),
            unary_expr,
        ),
        x,
        move |xs, (op, y)| Expr::new(ExprKind::Binary(op, Box::new(xs), Box::new(y))),
    )(i)
}

fn additive_expr(i: &str) -> IResult<&str, Expr> {
    let (i, x) = multiplicative_expr(i)?;

    fold_many0(
        pair(
            delimited(
                space0,
                alt((
                    value(BinaryOp::Add, char('+')),
                    value(BinaryOp::Sub, char('-')),
                )),
                space0,
            ),
            multiplicative_expr,
        ),
        x,
        move |xs, (op, y)| Expr::new(ExprKind::Binary(op, Box::new(xs), Box::new(y))),
    )(i)
}

fn equality_expr(i: &str) -> IResult<&str, Rel> {
    map(
        tuple((
            additive_expr,
            delimited(
                space0,
                alt((
                    value(EqualityOp::Eq, tag("==")),
                    value(EqualityOp::Ge, tag(">=")),
                    value(EqualityOp::Gt, char('>')),
                    value(EqualityOp::Le, tag("<=")),
                    value(EqualityOp::Lt, char('<')),
                )),
                space0,
            ),
            additive_expr,
        )),
        move |(x, op, y)| Rel::new(RelKind::Equality(op, Box::new(x), Box::new(y))),
    )(i)
}

fn rel_primary_expr(i: &str) -> IResult<&str, Rel> {
    alt((
        delimited(
            terminated(char('('), space0),
            or_expr,
            preceded(space0, cut(char(')'))),
        ),
        equality_expr,
    ))(i)
}

fn and_expr(i: &str) -> IResult<&str, Rel> {
    let (i, x) = rel_primary_expr(i)?;

    fold_many0(
        preceded(delimited(space0, tag("&&"), space0), rel_primary_expr),
        x,
        move |xs, y| Rel::new(RelKind::Binary(RelBinaryOp::And, Box::new(xs), Box::new(y))),
    )(i)
}

fn or_expr(i: &str) -> IResult<&str, Rel> {
    let (i, x) = and_expr(i)?;

    fold_many0(
        preceded(delimited(space0, tag("||"), space0), and_expr),
        x,
        move |xs, y| Rel::new(RelKind::Binary(RelBinaryOp::Or, Box::new(xs), Box::new(y))),
    )(i)
}

fn parse(i: &str) -> Option<Rel> {
    match or_expr(i) {
        Ok(("", x)) => Some(x),
        _ => None,
    }
}

pub trait Visitor
where
    Self: Sized,
{
    fn apply(mut self, rel: &Rel) -> Self {
        rel.accept(&mut self);
        self
    }

    fn visit_expr(&mut self, _: &Expr) {}
    fn visit_rel(&mut self, _: &Rel) {}
}

pub trait VisitorMut
where
    Self: Sized,
{
    fn apply(mut self, rel: &mut Rel) -> Self {
        rel.accept_mut(&mut self);
        self
    }

    fn visit_expr_mut(&mut self, _: &mut Expr) {}
    fn visit_rel_mut(&mut self, _: &mut Rel) {}
}

type SiteMap = HashMap<NodeId, Option<u8>>;

pub struct AssignNodeIdVisitor {
    next_id: NodeId,
    next_site: u8,
    site_map: SiteMap,
    visited_nodes: HashSet<Expr>,
}

impl AssignNodeIdVisitor {
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
            | Unary(Sign, _)
            | Unary(Tan, _)
            | Binary(Div, _, _)
            | Binary(Atan2, _, _))
    }
}

impl VisitorMut for AssignNodeIdVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
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

pub struct AssignSiteVisitor {
    site_map: SiteMap,
}

impl AssignSiteVisitor {
    pub fn new(site_map: SiteMap) -> Self {
        Self { site_map }
    }
}

impl VisitorMut for AssignSiteVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Some(site) = self.site_map.get(&expr.id) {
            expr.site = *site;
        }
    }
}

// Collects nodes (except the ones for x and y) sorted topologically.
pub struct NodeCollector {
    nodes: Vec<Expr>,
    next_node_id: NodeId,
}

impl NodeCollector {
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

impl Visitor for NodeCollector {
    fn visit_expr(&mut self, expr: &Expr) {
        if expr.id == self.next_node_id {
            self.nodes.push(expr.clone());
            self.next_node_id += 1;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Evaluator {
    rel_node: Rel,
    nodes: Vec<Expr>,
    vs: ValueStore,
}

impl Evaluator {
    pub fn new(rel_node: Rel, nodes: Vec<Expr>) -> Self {
        let n = nodes.len() + 2;
        Self {
            rel_node,
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
        self.rel_node.evaluate(&self.vs)
    }
}

#[derive(Clone, Debug)]
pub struct DynRelation {
    eval: Evaluator,
}

impl DynRelation {
    pub fn new(relation: &str) -> Self {
        let mut rel = parse(relation).unwrap();
        let v = AssignNodeIdVisitor::new().apply(&mut rel);
        AssignSiteVisitor::new(v.site_map()).apply(&mut rel);
        let v = NodeCollector::new().apply(&mut rel);
        Self {
            eval: Evaluator::new(rel, v.nodes()),
        }
    }

    pub fn evaluate(&mut self, x: TupperIntervalSet, y: TupperIntervalSet) -> EvaluationResult {
        self.eval.evaluate(x, y)
    }
}
