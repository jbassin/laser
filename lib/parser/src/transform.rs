use pest::Parser;
use pest::iterators::{Pair, Pairs};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LaserParser;

use crate::ast;

#[derive(Debug)]
struct LaserPair<'a, R: 'a> where R: pest::RuleType {
    rule: R,
    children: Pairs<'a, R>,
    start: usize,
    end: usize,
    string: String,
}

fn create_laser_pair<'a, R: 'a>(pair: Pair<'a, R>) -> LaserPair<R> where R: pest::RuleType {
    let rule = pair.as_rule();
    let span = pair.as_span();
    let start = span.start();
    let end = span.end();
    let string = span.as_str().to_string();
    let children = pair.into_inner();
    LaserPair {
        rule,
        children,
        start,
        end,
        string,
    }
}

fn parse_with_rule(rule: Rule, str: &'static str) -> Pair<Rule> {
    LaserParser::parse(rule, str).unwrap().next().unwrap()
}

fn literal(pair: Pair<Rule>) -> ast::Literal {
    use ast::Literal::*;

    let pair = create_laser_pair(pair);

    match &pair.rule {
        Rule::integer => Integer(pair.string.parse().unwrap()),
        Rule::float => Float(pair.string.parse().unwrap()),
        Rule::true_ => Boolean(true),
        Rule::false_ => Boolean(false),
        Rule::atom => Atom(pair.string.trim_start_matches(":").to_string()),
        Rule::variable => Variable(pair.string),
        _ => unreachable!("Missing literal")
    }
}

fn operator(pair: Pair<Rule>) -> ast::Operator {
    use ast::Operator::*;

    let pair = create_laser_pair(pair);

    match pair.string.as_str() {
        "+" => Add,
        "-" => Subtract,
        "*" => Multiply,
        "/" => Divide,
        "%" => Modulo,
        op => Custom(op.to_string())
    }
}

fn binary_operation(mut pair: LaserPair<Rule>) -> Box<ast::Expression> {
    use ast::Expression::*;

    let lhs = expression(pair.children.next().unwrap());
    let op = operator(pair.children.next().unwrap());
    let rhs = expression(pair.children.next().unwrap());
    Box::from(Binary(lhs, op, rhs))
}

fn expression(pair: Pair<Rule>) -> Box<ast::Expression> {
    use ast::Expression::*;

    let mut pair = create_laser_pair(pair);

    match &pair.rule {
        Rule::expression => expression(pair.children.next().unwrap()),
        Rule::literal => Box::from(Literal(literal(pair.children.next().unwrap()))),
        Rule::level_2_bin_op
        | Rule::level_1_bin_op => binary_operation(pair),
        _ => {
            println!("{:#?}", pair);
            unreachable!("Missing expression")
        },
    }
}

#[cfg(test)]
mod transform_tests {
    use super::*;
    use ast::Expression::*;
    use ast::Literal::*;
    use ast::Operator::*;

    #[test]
    fn literal_integer() {
        assert_eq!(
            literal(
                parse_with_rule(
                    Rule::integer,
                    "4"
                )
            ),
            Integer(4)
        );
    }

    #[test]
    fn literal_float() {
        assert_eq!(
            literal(
                parse_with_rule(
                    Rule::float,
                    "8."
                )
            ),
            Float(8.0)
        );
    }

    #[test]
    fn literal_true() {
        assert_eq!(
            literal(
                parse_with_rule(
                    Rule::true_,
                    "true"
                )
            ),
            Boolean(true)
        );
    }

    #[test]
    fn literal_false() {
        assert_eq!(
            literal(
                parse_with_rule(
                    Rule::false_,
                    "false"
                )
            ),
            Boolean(false)
        );
    }

    #[test]
    fn literal_atom() {
        assert_eq!(
            literal(
                parse_with_rule(
                    Rule::atom,
                    ":atom"
                )
            ),
            Atom("atom".to_string())
        );
    }

    #[test]
    fn literal_variable() {
        assert_eq!(
            literal(
                parse_with_rule(
                    Rule::variable,
                    "variable"
                )
            ),
            Variable("variable".to_string())
        );
    }

    #[test]
    fn expression_literal() {
        assert_eq!(
            expression(
                parse_with_rule(
                    Rule::expression,
                    "1"
                ).into_inner().next().unwrap()
            ),
            Box::new(Literal(Integer(1)))
        );
    }

    #[test]
    fn expression_addition() {
        assert_eq!(
            expression(
                parse_with_rule(
                    Rule::expression,
                    "1 + 2"
                ).into_inner().next().unwrap()
            ),
            Box::new(Binary(Box::new(Literal(Integer(1))), Add, Box::new(Literal(Integer(2)))))
        );
    }

    #[test]
    fn expression_compound() {
        assert_eq!(
            expression(
                parse_with_rule(
                    Rule::expression,
                    "(1 - 2) * 15 - 6 % 2"
                ).into_inner().next().unwrap()
            ),
            Box::new(
                Binary(
                    Box::new(
                        Binary(
                            Box::new(
                                Binary(
                                    Box::new(
                                        Literal(
                                            Integer(1)
                                        )
                                    ),
                                    Subtract,
                                    Box::new(
                                        Literal(
                                            Integer(2)
                                        )
                                    ),
                                )
                            ),
                            Multiply,
                            Box::new(
                                Literal(
                                    Integer(15)
                                )
                            ),
                        )
                    ),
                    Subtract,
                    Box::new(
                        Binary(
                            Box::new(
                                Literal(
                                    Integer(6)
                                )
                            ),
                            Modulo,
                            Box::new(
                                Literal(
                                    Integer(2)
                                )
                            ),
                        )
                    ),
                )
            )
        );
    }
}
