#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

#[macro_use] extern crate lazy_static;

mod ast;

pub fn test() -> String {
    "hilo".to_owned()
}

#[cfg(test)]
mod transform_tests {
    use super::*;
    use crate::ast::{Expression, Expression::*, Location, Operator};

    lazy_static! {
        static ref PARSER: grammar::ExprParser = grammar::ExprParser::new();
    }

    fn parse(token: &'static str) -> Box<Expression> {
        PARSER.parse(token).unwrap()
    }

    #[test]
    fn literal_integer() {
        assert_eq!(
            *parse("4"),
            Integer {
                location: Location::new(0, 1),
                value: "4".to_string()
            },
        );
    }

    #[test]
    fn literal_float() {
        assert_eq!(
            *parse("4."),
            Float {
                location: Location::new(0, 2),
                value: "4.".to_string()
            },
        );
    }

    #[test]
    fn literal_variable() {
        assert_eq!(
            *parse("foo_bar"),
            Variable {
                location: Location::new(0, 7),
                value: "foo_bar".to_string()
            },
        );
    }

    #[test]
    fn literal_atom() {
        assert_eq!(
            *parse(":atom"),
            Atom {
                location: Location::new(0, 5),
                value: "atom".to_string()
            },
        );
    }

    #[test]
    fn literal_true() {
        assert_eq!(
            *parse("(true)"),
            Boolean {
                location: Location::new(1, 5),
                value: "true".to_string()
            },
        );
    }

    #[test]
    fn literal_false() {
        assert_eq!(
            *parse("false"),
            Boolean {
                location: Location::new(0, 5),
                value: "false".to_string()
            },
        );
    }

    #[test]
    fn bin_op_pow() {
        assert_eq!(
            *parse("1 ** 2."),
            BinaryOperation {
                location: Location::new(0, 7),
                operator: Operator::Pow {
                    location: Location::new(2, 4),
                    value: "**".to_string()
                },
                left: Box::new(Integer {
                    location: Location::new(0, 1),
                    value: "1".to_string(),
                }),
                right: Box::new(Float {
                    location: Location::new(5, 7),
                    value: "2.".to_string(),
                }),
            }
        );
    }

    #[test]
    fn bin_op_level_1_custom() {
        assert_eq!(
            *parse("1 **> 2."),
            BinaryOperation {
                location: Location::new(0, 8),
                operator: Operator::Custom {
                    location: Location::new(2, 5),
                    value: "**>".to_string()
                },
                left: Box::new(Integer {
                    location: Location::new(0, 1),
                    value: "1".to_string(),
                }),
                right: Box::new(Float {
                    location: Location::new(6, 8),
                    value: "2.".to_string(),
                }),
            }
        );
    }
}