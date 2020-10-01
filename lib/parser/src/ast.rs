use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    start: usize,
    end: usize,
}

impl Location {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add {
        location: Location,
        value: String,
    },

    Sub {
        location: Location,
        value: String,
    },

    Mul {
        location: Location,
        value: String,
    },

    Div {
        location: Location,
        value: String,
    },

    Mod {
        location: Location,
        value: String,
    },

    Pow {
        location: Location,
        value: String,
    },

    Custom {
        location: Location,
        value: String,
    },
}

impl Operator {
    pub fn value(&self) -> &String {
        use Operator::*;

        match self {
            Add { value, .. } => value,
            Sub { value, .. } => value,
            Mul { value, .. } => value,
            Div { value, .. } => value,
            Mod { value, .. } => value,
            Pow { value, .. } => value,
            Custom { value, .. } => value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Integer {
        location: Location,
        value: String,
    },

    Float {
        location: Location,
        value: String,
    },

    Boolean {
        location: Location,
        value: String,
    },

    Atom {
        location: Location,
        value: String,
    },

    Variable {
        location: Location,
        value: String,
    },

    BinaryOperation {
        location: Location,
        operator: Operator,
        left: Box<Self>,
        right: Box<Self>,
    },
}
