use std::fmt;

#[derive(PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Atom(String),
    Variable(String),
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Literal::*;

        match self {
            Integer(i) => write!(f, "{:#?}", i),
            Float(f_) => write!(f, "{:#?}", f_),
            Boolean(b) => write!(f, "{:#?}", b),
            Atom(a) => write!(f, "{:#?}", a),
            Variable(v) => write!(f, "{:#?}", v),
        }
    }
}

#[derive(PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Custom(String),
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Operator::*;

        match self {
            Add => write!(f, "+"),
            Subtract => write!(f, "-"),
            Multiply => write!(f, "*"),
            Divide => write!(f, "/"),
            Modulo => write!(f, "%"),
            Custom(s) => write!(f, "{}", s),
        }
    }
}

#[derive(PartialEq)]
pub enum Expression {
    Binary(Box<Expression>, Operator, Box<Expression>),
    Literal(Literal),
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;

        match self {
            Binary(lhs, op, rhs) => write!(f, "({:#?} {:#?} {:#?})", lhs, op, rhs),
            Literal(l) => write!(f, "{:#?}", l)
        }
    }
}
