use pest::Parser;
use pest_derive::Parser;
use crate::lang::ast::*;
use anyhow::{anyhow, Result};

#[derive(Parser)]
#[grammar = "lang/grammar.pest"]
pub struct LangParser;

pub fn parse(input: &str) -> Result<Expr> {
    let mut pairs = LangParser::parse(Rule::program, input)
        .map_err(|e| anyhow!("Parse error: {}", e))?;

    let program = pairs.next().unwrap();
    let expr_pair = program.into_inner().next().unwrap();

    parse_expr(expr_pair)
}

fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    match pair.as_rule() {
        Rule::expr => {
            let inner = pair.into_inner().next().unwrap();
            parse_expr(inner)
        }
        Rule::let_expr => parse_let(pair),
        Rule::lambda => parse_lambda(pair),
        Rule::match_expr => parse_match(pair),
        Rule::if_expr => parse_if(pair),
        Rule::binary_expr => parse_binary(pair),
        Rule::comp_expr => parse_comp(pair),
        Rule::logic_expr => parse_logic(pair),
        Rule::cons_expr => parse_cons(pair),
        Rule::concat_expr => parse_concat(pair),
        Rule::add_expr => parse_add(pair),
        Rule::mul_expr => parse_mul(pair),
        Rule::pow_expr => parse_pow(pair),
        Rule::unary_expr => parse_unary(pair),
        Rule::app_expr => parse_app(pair),
        Rule::primary => parse_primary(pair),
        _ => Err(anyhow!("Unexpected rule: {:?}", pair.as_rule())),
    }
}

fn parse_let(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let value = parse_expr(inner.next().unwrap())?;
    let body = parse_expr(inner.next().unwrap())?;

    Ok(Expr::Let {
        name,
        value: Box::new(value),
        body: Box::new(body),
    })
}

fn parse_lambda(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();
    let param_list = inner.next().unwrap();
    let params: Vec<String> = param_list
        .into_inner()
        .map(|p| p.as_str().to_string())
        .collect();
    let body = parse_expr(inner.next().unwrap())?;

    Ok(Expr::Lambda {
        params,
        body: Box::new(body),
    })
}

fn parse_match(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();
    let expr = parse_expr(inner.next().unwrap())?;
    let match_arms = inner.next().unwrap();

    let arms: Result<Vec<MatchArm>> = match_arms
        .into_inner()
        .map(|arm_pair| {
            let mut arm_inner = arm_pair.into_inner();
            let pattern = parse_pattern(arm_inner.next().unwrap())?;
            let expr = parse_expr(arm_inner.next().unwrap())?;
            Ok(MatchArm { pattern, expr })
        })
        .collect();

    Ok(Expr::Match {
        expr: Box::new(expr),
        arms: arms?,
    })
}

fn parse_pattern(pair: pest::iterators::Pair<Rule>) -> Result<Pattern> {
    match pair.as_rule() {
        Rule::pattern => {
            let inner = pair.into_inner().next().unwrap();
            parse_pattern(inner)
        }
        Rule::ident => {
            let name = pair.as_str();
            if name == "_" {
                Ok(Pattern::Wildcard)
            } else {
                Ok(Pattern::Var(name.to_string()))
            }
        }
        Rule::number => {
            let n = pair.as_str().parse()?;
            Ok(Pattern::Number(n))
        }
        Rule::bool_lit => {
            let b = pair.as_str().parse()?;
            Ok(Pattern::Bool(b))
        }
        Rule::string_lit => {
            let s = pair.as_str();
            Ok(Pattern::String(s[1..s.len()-1].to_string()))
        }
        Rule::list_pattern => {
            let patterns: Result<Vec<Pattern>> = pair
                .into_inner()
                .map(parse_pattern)
                .collect();
            Ok(Pattern::List(patterns?))
        }
        Rule::cons_pattern => {
            let mut inner = pair.into_inner();
            let head_name = inner.next().unwrap().as_str().to_string();
            let tail = parse_pattern(inner.next().unwrap())?;
            Ok(Pattern::Cons {
                head: Box::new(Pattern::Var(head_name)),
                tail: Box::new(tail),
            })
        }
        _ => Err(anyhow!("Invalid pattern: {:?}", pair.as_rule())),
    }
}

fn parse_if(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();
    let cond = parse_expr(inner.next().unwrap())?;
    let then_branch = parse_expr(inner.next().unwrap())?;
    let else_branch = parse_expr(inner.next().unwrap())?;

    Ok(Expr::If {
        cond: Box::new(cond),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
    })
}

fn parse_binary_op<F>(pair: pest::iterators::Pair<Rule>, op_parser: F) -> Result<Expr>
where
    F: Fn(&str) -> Option<BinOp>,
{
    let mut inner = pair.into_inner();
    let mut left = parse_expr(inner.next().unwrap())?;

    while let Some(op_pair) = inner.next() {
        let op_str = op_pair.as_str();
        let op = op_parser(op_str)
            .ok_or_else(|| anyhow!("Unknown operator: {}", op_str))?;
        let right = parse_expr(inner.next().unwrap())?;

        left = Expr::BinOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_binary(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    parse_binary_op(pair, |s| match s {
        ">>" => Some(BinOp::PipeForward),
        "<<" => Some(BinOp::PipeBackward),
        _ => None,
    })
}

fn parse_comp(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    parse_binary_op(pair, |s| match s {
        "==" => Some(BinOp::Eq),
        "!=" => Some(BinOp::Neq),
        "<" => Some(BinOp::Lt),
        ">" => Some(BinOp::Gt),
        "<=" => Some(BinOp::Lte),
        ">=" => Some(BinOp::Gte),
        _ => None,
    })
}

fn parse_logic(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    parse_binary_op(pair, |s| match s {
        "&&" => Some(BinOp::And),
        "||" => Some(BinOp::Or),
        _ => None,
    })
}

fn parse_cons(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    parse_binary_op(pair, |s| match s {
        "::" => Some(BinOp::Cons),
        _ => None,
    })
}

fn parse_concat(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    parse_binary_op(pair, |s| match s {
        "++" => Some(BinOp::Concat),
        _ => None,
    })
}

fn parse_add(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    parse_binary_op(pair, |s| match s {
        "+" => Some(BinOp::Add),
        "-" => Some(BinOp::Sub),
        _ => None,
    })
}

fn parse_mul(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    parse_binary_op(pair, |s| match s {
        "*" => Some(BinOp::Mul),
        "/" => Some(BinOp::Div),
        "%" => Some(BinOp::Mod),
        _ => None,
    })
}

fn parse_pow(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    parse_binary_op(pair, |s| match s {
        "^" => Some(BinOp::Pow),
        _ => None,
    })
}

fn parse_unary(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    match first.as_rule() {
        Rule::neg_op => {
            let expr = parse_expr(inner.next().unwrap())?;
            Ok(Expr::UnOp {
                op: UnOp::Neg,
                expr: Box::new(expr),
            })
        }
        _ => parse_expr(first),
    }
}

fn parse_app(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();
    let func = parse_expr(inner.next().unwrap())?;

    let args: Result<Vec<Expr>> = inner.map(parse_expr).collect();
    let args = args?;

    if args.is_empty() {
        Ok(func)
    } else {
        Ok(Expr::App {
            func: Box::new(func),
            args,
        })
    }
}

fn parse_primary(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::expr => parse_expr(inner),
        Rule::number => {
            let n = inner.as_str().parse()?;
            Ok(Expr::Number(n))
        }
        Rule::bool_lit => {
            let b = inner.as_str().parse()?;
            Ok(Expr::Bool(b))
        }
        Rule::string_lit => {
            let s = inner.as_str();
            Ok(Expr::String(s[1..s.len()-1].to_string()))
        }
        Rule::ident => Ok(Expr::Var(inner.as_str().to_string())),
        Rule::list => {
            let exprs: Result<Vec<Expr>> = inner.into_inner().map(parse_expr).collect();
            Ok(Expr::List(exprs?))
        }
        Rule::range => {
            let mut range_inner = inner.into_inner();
            let start: i64 = range_inner.next().unwrap().as_str().parse()?;
            let end: i64 = range_inner.next().unwrap().as_str().parse()?;
            Ok(Expr::Range { start, end })
        }
        Rule::list_comp => {
            let mut comp_inner = inner.into_inner();
            let expr = parse_expr(comp_inner.next().unwrap())?;
            let var = comp_inner.next().unwrap().as_str().to_string();
            let list = parse_expr(comp_inner.next().unwrap())?;

            let guards: Result<Vec<Expr>> = comp_inner
                .map(|guard| parse_expr(guard.into_inner().next().unwrap()))
                .collect();

            Ok(Expr::ListComp {
                expr: Box::new(expr),
                var,
                list: Box::new(list),
                guards: guards?,
            })
        }
        _ => Err(anyhow!("Unexpected primary: {:?}", inner.as_rule())),
    }
}
