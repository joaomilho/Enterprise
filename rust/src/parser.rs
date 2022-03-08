use pest::iterators::{Pair, Pairs};
use pest::Parser;

#[derive(Parser)]
#[grammar = "e.pest"]
pub struct EParser;

#[derive(Debug, Eq, PartialEq)]
pub enum VarType {
    r#String,
    Boolean,
    Money,
    Null,
}

fn to_var_type(var_type: &str) -> VarType {
    match var_type {
        "String" => VarType::r#String,
        "Boolean" => VarType::Boolean,
        "Money" => VarType::Money,
        "Null" => VarType::Null,
        _ => unreachable!("No such var type"),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum OpType {
    GT,
    GTE,
    LT,
    LTE,
    EQ,
    NEQ,
    NOT,
    AND,
    OR,
    MUL,
    DIV,
    SUM,
    SUB,
    MOD,
}

fn to_op_type(op_type: &str) -> OpType {
    match op_type {
        ">" => OpType::GT,
        ">=" => OpType::GTE,
        "<" => OpType::LT,
        "<=" => OpType::LTE,
        "==" => OpType::EQ,
        "!=" => OpType::NEQ,
        "!" => OpType::NOT,
        "&&" => OpType::AND,
        "||" => OpType::OR,
        "*" => OpType::MUL,
        "/" => OpType::DIV,
        "+" => OpType::SUM,
        "-" => OpType::SUB,
        "%" => OpType::MOD,
        _ => unreachable!("No such op type"),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MutType {
    INC,
    DEC,
}

fn to_mut_type(mut_type: &str) -> MutType {
    match mut_type {
        "++" => MutType::INC,
        "--" => MutType::DEC,
        _ => unreachable!("No such mut type"),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum E {
    Lib(String),
    LitString(String),
    LitMoney(usize),
    LitNull,
    LitBool(bool),
    Id(String),
    Decl(VarType, String, Box<E>),
    If(Box<E>, Vec<E>),
    IfElse(Box<E>, Vec<E>, Vec<E>),
    While(Box<E>, Vec<E>),
    Mutation(String, MutType),
    Call(String, Vec<E>),
    Op(Box<E>, OpType, Box<E>),
}

impl E {
    fn call(callee: &str, params: Vec<E>) -> E {
        E::Call(String::from(callee), params)
    }

    fn if_else(cond: E, if_block: Vec<E>, else_block: Vec<E>) -> E {
        E::IfElse(Box::new(cond), if_block, else_block)
    }

    fn r#if(cond: E, if_block: Vec<E>) -> E {
        E::If(Box::new(cond), if_block)
    }

    fn mutation(var: &str, mutation: &str) -> E {
        E::Mutation(String::from(var), to_mut_type(mutation))
    }

    fn lib(s: &str) -> E {
        E::Lib(String::from(s))
    }

    fn lit_string(s: &str) -> E {
        E::LitString(String::from(s))
    }

    fn lit_money(v: usize) -> E {
        E::LitMoney(v)
    }

    fn lit_null() -> E {
        E::LitNull
    }

    fn lit_bool(b: bool) -> E {
        E::LitBool(b)
    }

    fn id(s: &str) -> E {
        E::Id(String::from(s))
    }

    fn decl(var_type: &str, name: &str, expr: E) -> E {
        E::Decl(to_var_type(var_type), String::from(name), Box::new(expr))
    }

    fn op(l: E, oper: &str, r: E) -> E {
        E::Op(Box::new(l), to_op_type(oper), Box::new(r))
    }

    fn r#while(cond: E, exprs: Vec<E>) -> E {
        E::While(Box::new(cond), exprs)
    }
}

pub fn to_ast(code: &str) -> Vec<E> {
    fn pairs_to_ast(pair: Pairs<Rule>) -> Vec<E> {
        return pair.map(pair_to_ast).collect();
    }

    fn option_to_ast(pair: Option<Pair<Rule>>) -> Vec<E> {
        match pair {
            Some(list) => list.into_inner().map(pair_to_ast).collect(),
            None => vec![],
        }
    }

    fn to_multiplier(pair: Option<Pair<Rule>>) -> usize {
        match pair {
            Some(valuation) => match valuation.as_str() {
                "k" => 1000,
                "M" => 1000 * 1000,
                "B" => 1000 * 1000 * 1000,
                "T" => 1000 * 1000 * 1000 * 1000,
                _ => 1,
            },
            None => 1,
        }
    }

    fn pair_to_ast(pair: Pair<Rule>) -> E {
        match pair.as_rule() {
            Rule::lib => E::lib(pair.as_str()),
            Rule::string => E::lit_string(pair.as_str()),
            Rule::money => {
                let mut inner = pair.into_inner();
                let value: usize = inner.next().unwrap().as_str().parse().expect("NAN");
                let multiplier = to_multiplier(inner.next());

                return E::lit_money(value * multiplier);
            }
            Rule::null => E::lit_null(),
            Rule::bool => E::lit_bool(pair.as_str() == "true"),

            Rule::var_name => E::id(pair.as_str()),
            Rule::decl => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let r#type = inner.next().unwrap().as_str();
                let value = inner.next().unwrap();
                E::decl(name, r#type, pair_to_ast(value))
            }
            Rule::mutate => {
                let mut inner = pair.into_inner();

                let name = inner.next().unwrap().as_str();
                let mutation = inner.next().unwrap().as_str();

                return E::mutation(name, mutation);
            }
            Rule::call => {
                let mut inner = pair.into_inner();

                let callee = inner.next().unwrap().as_str();

                let args = option_to_ast(inner.next());
                return E::call(callee, args);
            }

            Rule::if_clause => {
                let mut inner = pair.into_inner();
                let testable = inner.next().unwrap();

                let statements = inner.next().unwrap().into_inner();

                return E::r#if(pair_to_ast(testable), pairs_to_ast(statements));
            }

            Rule::if_else_clause => {
                let mut inner = pair.into_inner();
                let testable = inner.next().unwrap();

                let if_block = inner.next().unwrap().into_inner();
                let else_block = inner.next().unwrap().into_inner();

                return E::if_else(
                    pair_to_ast(testable),
                    pairs_to_ast(if_block),
                    pairs_to_ast(else_block),
                );
            }

            Rule::while_clause => {
                let mut inner = pair.into_inner();
                let testable = inner.next().unwrap();

                let statements = inner.next().unwrap().into_inner();

                return E::r#while(pair_to_ast(testable), pairs_to_ast(statements));
            }

            Rule::testable => {
                let mut inner = pair.into_inner();
                let len = inner.clone().count();
                match len {
                    3 => {
                        return E::op(
                            pair_to_ast(inner.next().unwrap()),
                            inner.next().unwrap().as_str(),
                            pair_to_ast(inner.next().unwrap()),
                        );
                    }
                    _ => pair_to_ast(inner.next().unwrap()),
                }
            }

            _ => unreachable!(pair),
        }
    }

    let parsed = EParser::parse(Rule::program, &code).expect("unsuccessful parse");

    pairs_to_ast(parsed)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_to_ast() {
        let fizz =
            fs::read_to_string("src/samples/fdcFizzBuzzDelegator.E™").expect("Cannot read fizz");
        let uni =
            fs::read_to_string("src/samples/fdcUnicornEvaluator.E™").expect("Cannot read uni");

        assert_eq!(
            to_ast(&uni),
            vec![
                E::lib("IO.read.delegator.dlIOReadDelegator"),
                E::lib("IO.write.delegator.dlIOWriteDelegator"),
                E::lib("String.contains.delegator.dlStringContainsDelegator"),
                E::decl(
                    "String",
                    "answer",
                    E::call("read", vec![E::lit_string("Tell us your idea: ")])
                ),
                E::if_else(
                    E::call(
                        "contains",
                        vec![E::id("answer"), E::lit_string("Blockchain")]
                    ),
                    vec![E::call(
                        "write",
                        vec![E::lit_string("Disruptive. 🦄🦄🦄🦄🦄")]
                    )],
                    vec![E::if_else(
                        E::call("contains", vec![E::id("answer"), E::lit_string("Tinder")]),
                        vec![E::call(
                            "write",
                            vec![E::lit_string("Pain killer. 🦄🦄🦄🦄")]
                        )],
                        vec![E::if_else(
                            E::call("contains", vec![E::id("answer"), E::lit_string("Cloud")]),
                            vec![E::call("write", vec![E::lit_string("Vitamin. 🦄🦄🦄")])],
                            vec![E::if_else(
                                E::call(
                                    "contains",
                                    vec![E::id("answer"), E::lit_string("Facebook")]
                                ),
                                vec![E::call(
                                    "write",
                                    vec![E::lit_string("Will sleep on that. 🦄🦄")]
                                )],
                                vec![E::if_else(
                                    E::call(
                                        "contains",
                                        vec![E::id("answer"), E::lit_string("Chat")]
                                    ),
                                    vec![E::call("write", vec![E::lit_string("Meh. 🦄")])],
                                    vec![E::call("write", vec![E::lit_string("Cockroach.")])]
                                )]
                            )]
                        )]
                    )]
                )
            ]
        );

        assert_eq!(
            to_ast(&fizz),
            vec![
                E::lib("IO.write.delegator.dlIOWriteDelegator"),
                E::decl("Money", "x", E::lit_money(0)),
                E::decl("String", "out", E::lit_string("")),
                E::r#while(
                    E::op(E::id("x"), "<", E::lit_money(1000)),
                    vec![
                        E::if_else(
                            E::op(
                                E::op(
                                    E::op(E::id("x"), "%", E::lit_money(5)),
                                    "==",
                                    E::lit_money(0)
                                ),
                                "&&",
                                E::op(
                                    E::op(E::id("x"), "%", E::lit_money(3)),
                                    "==",
                                    E::lit_money(0)
                                ),
                            ),
                            vec![E::call("write", vec![E::lit_string("Fizz Buzz")])],
                            vec![E::if_else(
                                E::op(
                                    E::op(E::id("x"), "%", E::lit_money(3)),
                                    "==",
                                    E::lit_money(0)
                                ),
                                vec![E::call("write", vec![E::lit_string("Fizz")])],
                                vec![E::if_else(
                                    E::op(
                                        E::op(E::id("x"), "%", E::lit_money(5)),
                                        "==",
                                        E::lit_money(0)
                                    ),
                                    vec![E::call("write", vec![E::lit_string("Buzz")])],
                                    vec![E::call("write", vec![E::id("x")])]
                                )]
                            )]
                        ),
                        E::mutation("x", "++")
                    ]
                )
            ]
        )
    }
}
