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
    Floor,
    Log,
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
enum Expr {
    Constant(TupperIntervalSet),
    X,
    Y,
    Unary(UnaryOp, Box<Node>),
    Binary(BinaryOp, Box<Node>, Box<Node>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum RelExpr {
    Equality(EqualityOp, Box<Node>, Box<Node>),
    Binary(RelBinaryOp, Box<RelNode>, Box<RelNode>),
}

#[derive(Clone, Debug)]
pub struct Node {
    expr: Expr,
    site: Option<u8>,
    value_index: usize,
}

impl PartialEq for Node {
    fn eq(&self, rhs: &Self) -> bool {
        self.expr == rhs.expr
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.expr.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct RelNode {
    expr: RelExpr,
}

impl PartialEq for RelNode {
    fn eq(&self, rhs: &Self) -> bool {
        self.expr == rhs.expr
    }
}

impl Eq for RelNode {}

impl Hash for RelNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.expr.hash(state);
    }
}

type ValueStore = Vec<TupperIntervalSet>;

impl Node {
    fn new(expr: Expr) -> Self {
        Self {
            expr,
            site: None,
            value_index: 0,
        }
    }

    fn accept<V: Visitor>(&self, visitor: &mut V) {
        use Expr::*;
        match &self.expr {
            Unary(_, x) => x.accept(visitor),
            Binary(_, x, y) => {
                x.accept(visitor);
                y.accept(visitor);
            }
            _ => (),
        };
        visitor.visit_node(self);
    }

    fn accept_mut<V: VisitorMut>(&mut self, visitor: &mut V) {
        use Expr::*;
        match &mut self.expr {
            Unary(_, x) => x.accept_mut(visitor),
            Binary(_, x, y) => {
                x.accept_mut(visitor);
                y.accept_mut(visitor);
            }
            _ => (),
        };
        visitor.visit_node_mut(self);
    }

    fn evaluate(&self, vs: &ValueStore) -> TupperIntervalSet {
        use {BinaryOp::*, Expr::*, UnaryOp::*};
        match &self.expr {
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
            Unary(Floor, x) => x.value(vs).floor(self.site),
            Unary(Log, x) => x.value(vs).log(),
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
        &vs[self.value_index]
    }
}

impl RelNode {
    fn new(expr: RelExpr) -> Self {
        Self { expr }
    }

    fn accept<V: Visitor>(&self, visitor: &mut V) {
        use RelExpr::*;
        match &self.expr {
            Equality(_, x, y) => {
                x.accept(visitor);
                y.accept(visitor);
            }
            Binary(_, x, y) => {
                x.accept(visitor);
                y.accept(visitor);
            }
        };
        visitor.visit_rel_node(self);
    }

    fn accept_mut<V: VisitorMut>(&mut self, visitor: &mut V) {
        use RelExpr::*;
        match &mut self.expr {
            Equality(_, x, y) => {
                x.accept_mut(visitor);
                y.accept_mut(visitor);
            }
            Binary(_, x, y) => {
                x.accept_mut(visitor);
                y.accept_mut(visitor);
            }
        };
        visitor.visit_rel_node_mut(self);
    }

    fn evaluate(&self, vs: &ValueStore) -> EvaluationResult {
        use {EqualityOp::*, RelBinaryOp::*, RelExpr::*};
        match &self.expr {
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

fn primary_expr(i: &str) -> IResult<&str, Node> {
    alt((
        map(decimal_literal, |s| {
            let s = ["[", s, ",", s, "]"].concat();
            let x = TupperIntervalSet::from(dec_interval!(&s).unwrap());
            Node::new(Expr::Constant(x))
        }),
        map(tag("pi"), |_| {
            let x = TupperIntervalSet::from(DecoratedInterval::PI);
            Node::new(Expr::Constant(x))
        }),
        map(char('e'), |_| {
            let x = TupperIntervalSet::from(DecoratedInterval::E);
            Node::new(Expr::Constant(x))
        }),
        value(Node::new(Expr::X), char('x')),
        value(Node::new(Expr::Y), char('y')),
        delimited(
            terminated(char('('), space0),
            additive_expr,
            preceded(space0, cut(char(')'))),
        ),
    ))(i)
}

fn fn1(i: &str) -> IResult<&str, UnaryOp> {
    alt((
        value(UnaryOp::Acosh, tag("acosh")),
        value(UnaryOp::Asinh, tag("asinh")),
        value(UnaryOp::Atanh, tag("atanh")),
        value(UnaryOp::Floor, tag("floor")),
        value(UnaryOp::Acos, tag("acos")),
        value(UnaryOp::Asin, tag("asin")),
        value(UnaryOp::Atan, tag("atan")),
        value(UnaryOp::Ceil, tag("ceil")),
        value(UnaryOp::Cosh, tag("cosh")),
        value(UnaryOp::Sign, tag("sign")),
        value(UnaryOp::Sinh, tag("sinh")),
        value(UnaryOp::Sqrt, tag("sqrt")),
        value(UnaryOp::Tanh, tag("tanh")),
        value(UnaryOp::Abs, tag("abs")),
        value(UnaryOp::Cos, tag("cos")),
        value(UnaryOp::Exp, tag("exp")),
        value(UnaryOp::Log, tag("log")),
        value(UnaryOp::Sin, tag("sin")),
        value(UnaryOp::Tan, tag("tan")),
    ))(i)
}

fn fn2(i: &str) -> IResult<&str, BinaryOp> {
    alt((
        value(BinaryOp::Atan2, tag("atan")),
        value(BinaryOp::Max, tag("max")),
        value(BinaryOp::Min, tag("min")),
    ))(i)
}

fn postfix_expr(i: &str) -> IResult<&str, Node> {
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
            move |(f, x)| Node::new(Expr::Unary(f, Box::new(x))),
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
            move |(f, (x, y))| Node::new(Expr::Binary(f, Box::new(x), Box::new(y))),
        ),
        primary_expr,
    ))(i)
}

fn unary_expr(i: &str) -> IResult<&str, Node> {
    alt((
        map(
            separated_pair(value(UnaryOp::Neg, char('-')), space0, unary_expr),
            |(op, x)| Node::new(Expr::Unary(op, Box::new(x))),
        ),
        postfix_expr,
    ))(i)
}

fn multiplicative_expr(i: &str) -> IResult<&str, Node> {
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
        move |xs, (op, y)| Node::new(Expr::Binary(op, Box::new(xs), Box::new(y))),
    )(i)
}

fn additive_expr(i: &str) -> IResult<&str, Node> {
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
        move |xs, (op, y)| Node::new(Expr::Binary(op, Box::new(xs), Box::new(y))),
    )(i)
}

fn equality_expr(i: &str) -> IResult<&str, RelNode> {
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
        move |(x, op, y)| RelNode::new(RelExpr::Equality(op, Box::new(x), Box::new(y))),
    )(i)
}

fn rel_primary_expr(i: &str) -> IResult<&str, RelNode> {
    alt((
        delimited(
            terminated(char('('), space0),
            or_expr,
            preceded(space0, cut(char(')'))),
        ),
        equality_expr,
    ))(i)
}

fn and_expr(i: &str) -> IResult<&str, RelNode> {
    let (i, x) = rel_primary_expr(i)?;

    fold_many0(
        preceded(delimited(space0, tag("&&"), space0), rel_primary_expr),
        x,
        move |xs, y| RelNode::new(RelExpr::Binary(RelBinaryOp::And, Box::new(xs), Box::new(y))),
    )(i)
}

fn or_expr(i: &str) -> IResult<&str, RelNode> {
    let (i, x) = and_expr(i)?;

    fold_many0(
        preceded(delimited(space0, tag("||"), space0), and_expr),
        x,
        move |xs, y| RelNode::new(RelExpr::Binary(RelBinaryOp::Or, Box::new(xs), Box::new(y))),
    )(i)
}

fn parse(i: &str) -> Option<RelNode> {
    match or_expr(i) {
        Ok(("", x)) => Some(x),
        _ => None,
    }
}

pub trait Visitor
where
    Self: Sized,
{
    fn apply(mut self, node: &RelNode) -> Self {
        node.accept(&mut self);
        self
    }

    fn visit_node(&mut self, _: &Node) {}
    fn visit_rel_node(&mut self, _: &RelNode) {}
}

pub trait VisitorMut
where
    Self: Sized,
{
    fn apply(mut self, node: &mut RelNode) -> Self {
        node.accept_mut(&mut self);
        self
    }

    fn visit_node_mut(&mut self, _: &mut Node) {}
    fn visit_rel_node_mut(&mut self, _: &mut RelNode) {}
}

pub struct AssignValueIndexVisitor {
    next_index: usize,
    next_site: u8,
    site_map: HashMap<usize, Option<u8>>,
    visited_nodes: HashSet<Node>,
}

impl AssignValueIndexVisitor {
    pub fn new() -> Self {
        Self {
            next_index: 2, // 0 for x, 1 for y
            next_site: 0,
            site_map: HashMap::new(),
            visited_nodes: HashSet::new(),
        }
    }

    pub fn site_map(self) -> HashMap<usize, Option<u8>> {
        self.site_map
    }

    fn expr_can_perform_cut(expr: &Expr) -> bool {
        use BinaryOp::*;
        use Expr::*;
        use UnaryOp::*;
        matches!(expr,
            Unary(Ceil, _)
            | Unary(Floor, _)
            | Unary(Sign, _)
            | Unary(Tan, _)
            | Binary(Div, _, _)
            | Binary(Atan2, _, _))
    }
}

impl VisitorMut for AssignValueIndexVisitor {
    fn visit_node_mut(&mut self, node: &mut Node) {
        match self.visited_nodes.get(node) {
            Some(visited) => {
                node.value_index = visited.value_index;

                if let Some(site) = self.site_map.get_mut(&node.value_index) {
                    if site.is_none() && self.next_site <= 31 {
                        *site = Some(self.next_site as u8);
                        self.next_site += 1;
                    }
                }
            }
            _ => {
                node.value_index = match &node.expr {
                    Expr::X => 0,
                    Expr::Y => 1,
                    _ => {
                        let i = self.next_index;
                        self.next_index += 1;
                        i
                    }
                };

                if Self::expr_can_perform_cut(&node.expr) {
                    self.site_map.insert(node.value_index, None);
                }

                self.visited_nodes.insert(node.clone());
            }
        }
    }
}

pub struct AssignSiteVisitor {
    site_map: HashMap<usize, Option<u8>>,
}

impl AssignSiteVisitor {
    pub fn new(site_map: HashMap<usize, Option<u8>>) -> Self {
        Self { site_map }
    }
}

impl VisitorMut for AssignSiteVisitor {
    fn visit_node_mut(&mut self, node: &mut Node) {
        if let Some(site) = self.site_map.get(&node.value_index) {
            node.site = *site;
        }
    }
}

// Collects nodes (except the ones for x and y) sorted topologically.
pub struct NodeCollector {
    nodes: Vec<Node>,
    next_index: usize,
}

impl NodeCollector {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            next_index: 2, // 0 for x, 1 for y
        }
    }

    pub fn nodes(self) -> Vec<Node> {
        self.nodes
    }
}

impl Visitor for NodeCollector {
    fn visit_node(&mut self, node: &Node) {
        if node.value_index == self.next_index {
            self.nodes.push(node.clone());
            self.next_index += 1;
        }
    }
}

pub struct Evaluator {
    rel_node: RelNode,
    nodes: Vec<Node>,
    vs: ValueStore,
}

impl Evaluator {
    pub fn new(rel_node: RelNode, nodes: Vec<Node>) -> Self {
        let n = nodes.len() + 2;
        Self {
            rel_node,
            nodes,
            vs: vec![TupperIntervalSet::new(); n],
        }
    }

    pub fn eval(&mut self, x: TupperIntervalSet, y: TupperIntervalSet) -> EvaluationResult {
        self.vs[0] = x;
        self.vs[1] = y;
        for i in 0..self.nodes.len() {
            self.vs[i + 2] = self.nodes[i].evaluate(&self.vs);
        }
        self.rel_node.evaluate(&self.vs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hogehoge() {
        let mut rel = parse("asinh(x) + floor(y) == sin(floor(x) + y)").unwrap();
        let v = AssignValueIndexVisitor::new().apply(&mut rel);
        AssignSiteVisitor::new(v.site_map()).apply(&mut rel);
        let v = NodeCollector::new().apply(&mut rel);
        dbg!(&rel);

        let mut eval = Evaluator::new(rel, v.nodes());
        let x = TupperIntervalSet::from(const_dec_interval!(1.0, 2.0));
        let y = TupperIntervalSet::from(const_dec_interval!(1.0, 2.0));
        dbg!(eval.eval(x, y));

        assert!(false);
    }
}
