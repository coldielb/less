use crate::lang::ast::*;
use std::collections::HashMap;
use std::rc::Rc;
use anyhow::{anyhow, Result};

const MAX_CALL_DEPTH: usize = 10000;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Function {
        params: Vec<String>,
        body: Rc<Expr>,
        env: Rc<Env>,
    },
    Builtin(String),
    Thunk {
        expr: Rc<Expr>,
        env: Rc<Env>,
    },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            _ => false,
        }
    }
}

impl Value {
    pub fn to_string_repr(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => format!("\"{}\"", s),
            Value::List(items) => {
                let strs: Vec<String> = items.iter().map(|v| v.to_string_repr()).collect();
                format!("[{}]", strs.join(", "))
            }
            Value::Function { .. } => "<function>".to_string(),
            Value::Builtin(name) => format!("<builtin: {}>", name),
            Value::Thunk { .. } => "<thunk>".to_string(),
        }
    }
}

pub type Env = HashMap<String, Value>;

pub struct Interpreter {
    call_depth: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { call_depth: 0 }
    }

    fn check_depth(&self) -> Result<()> {
        if self.call_depth > MAX_CALL_DEPTH {
            Err(anyhow!("Maximum recursion depth exceeded"))
        } else {
            Ok(())
        }
    }

    pub fn eval(&mut self, expr: &Expr, env: &Rc<Env>) -> Result<Value> {
        self.check_depth()?;
        self.call_depth += 1;

        let result = match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::List(items) => {
                let values: Result<Vec<Value>> = items.iter()
                    .map(|item| self.eval(item, env))
                    .collect();
                Ok(Value::List(values?))
            }
            Expr::Var(name) => {
                env.get(name)
                    .cloned()
                    .ok_or_else(|| anyhow!("Undefined variable: {}", name))
                    .and_then(|v| self.force(v, env))
            }
            Expr::Lambda { params, body } => Ok(Value::Function {
                params: params.clone(),
                body: Rc::new((**body).clone()),
                env: env.clone(),
            }),
            Expr::App { func, args } => self.eval_app(func, args, env),
            Expr::Let { name, value, body } => {
                let thunk = Value::Thunk {
                    expr: Rc::new((**value).clone()),
                    env: env.clone(),
                };
                let mut new_env = (**env).clone();
                new_env.insert(name.clone(), thunk);
                self.eval(body, &Rc::new(new_env))
            }
            Expr::If { cond, then_branch, else_branch } => {
                let cond_val = self.eval(cond, env)?;
                match cond_val {
                    Value::Bool(true) => self.eval(then_branch, env),
                    Value::Bool(false) => self.eval(else_branch, env),
                    _ => Err(anyhow!("Condition must be a boolean")),
                }
            }
            Expr::BinOp { op, left, right } => self.eval_binop(*op, left, right, env),
            Expr::UnOp { op: UnOp::Neg, expr } => {
                let val = self.eval(expr, env)?;
                match val {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    _ => Err(anyhow!("Cannot negate non-number")),
                }
            }
            Expr::Range { start, end } => {
                let values: Vec<Value> = (*start..=*end)
                    .map(Value::Number)
                    .collect();
                Ok(Value::List(values))
            }
            Expr::ListComp { expr, var, list, guards } => {
                self.eval_list_comp(expr, var, list, guards, env)
            }
            Expr::Match { expr, arms } => self.eval_match(expr, arms, env),
        };

        self.call_depth -= 1;
        result
    }

    fn force(&mut self, value: Value, _env: &Rc<Env>) -> Result<Value> {
        match value {
            Value::Thunk { expr, env } => self.eval(&expr, &env),
            v => Ok(v),
        }
    }

    fn eval_app(&mut self, func_expr: &Expr, args: &[Expr], env: &Rc<Env>) -> Result<Value> {
        let func = self.eval(func_expr, env)?;

        match func {
            Value::Function { params, body, env: func_env } => {
                if args.len() < params.len() {
                    // Partial application
                    let applied_args = args.len();
                    let remaining_params = params[applied_args..].to_vec();

                    let mut new_env = (*func_env).clone();
                    for (param, arg) in params[..applied_args].iter().zip(args.iter()) {
                        let val = self.eval(arg, env)?;
                        new_env.insert(param.clone(), val);
                    }

                    Ok(Value::Function {
                        params: remaining_params,
                        body,
                        env: Rc::new(new_env),
                    })
                } else if args.len() == params.len() {
                    // Full application
                    let mut new_env = (*func_env).clone();
                    for (param, arg) in params.iter().zip(args.iter()) {
                        let val = self.eval(arg, env)?;
                        new_env.insert(param.clone(), val);
                    }
                    self.eval(&body, &Rc::new(new_env))
                } else {
                    // Over-application
                    let mut new_env = (*func_env).clone();
                    for (param, arg) in params.iter().zip(args.iter()) {
                        let val = self.eval(arg, env)?;
                        new_env.insert(param.clone(), val);
                    }
                    let result = self.eval(&body, &Rc::new(new_env))?;
                    let remaining_args = &args[params.len()..];

                    if remaining_args.is_empty() {
                        Ok(result)
                    } else {
                        self.eval_app(&Expr::Var("_result".to_string()), remaining_args,
                            &Rc::new(vec![("_result".to_string(), result)].into_iter().collect()))
                    }
                }
            }
            Value::Builtin(name) => self.eval_builtin(&name, args, env),
            _ => Err(anyhow!("Cannot call non-function")),
        }
    }

    fn eval_builtin(&mut self, name: &str, args: &[Expr], env: &Rc<Env>) -> Result<Value> {
        match name {
            "map" => {
                if args.len() < 2 {
                    return Err(anyhow!("map requires 2 arguments"));
                }
                let f = self.eval(&args[0], env)?;
                let list = self.eval(&args[1], env)?;

                match list {
                    Value::List(items) => {
                        let results: Result<Vec<Value>> = items.into_iter()
                            .map(|item| {
                                let item_expr = value_to_expr(&item)?;
                                self.eval_app(&value_to_expr(&f)?, &[item_expr], env)
                            })
                            .collect();
                        Ok(Value::List(results?))
                    }
                    _ => Err(anyhow!("map: second argument must be a list")),
                }
            }
            "filter" => {
                if args.len() < 2 {
                    return Err(anyhow!("filter requires 2 arguments"));
                }
                let f = self.eval(&args[0], env)?;
                let list = self.eval(&args[1], env)?;

                match list {
                    Value::List(items) => {
                        let mut results = Vec::new();
                        for item in items {
                            let item_expr = value_to_expr(&item)?;
                            let pred = self.eval_app(&value_to_expr(&f)?, &[item_expr], env)?;
                            match pred {
                                Value::Bool(true) => results.push(item),
                                Value::Bool(false) => {},
                                _ => return Err(anyhow!("filter: predicate must return bool")),
                            }
                        }
                        Ok(Value::List(results))
                    }
                    _ => Err(anyhow!("filter: second argument must be a list")),
                }
            }
            "fold" | "foldl" => {
                if args.len() < 3 {
                    return Err(anyhow!("{} requires 3 arguments", name));
                }
                let f = self.eval(&args[0], env)?;
                let mut acc = self.eval(&args[1], env)?;
                let list = self.eval(&args[2], env)?;

                match list {
                    Value::List(items) => {
                        for item in items {
                            let acc_expr = value_to_expr(&acc)?;
                            let item_expr = value_to_expr(&item)?;
                            acc = self.eval_app(&value_to_expr(&f)?, &[acc_expr, item_expr], env)?;
                        }
                        Ok(acc)
                    }
                    _ => Err(anyhow!("{}: third argument must be a list", name)),
                }
            }
            "foldr" => {
                if args.len() < 3 {
                    return Err(anyhow!("foldr requires 3 arguments"));
                }
                let f = self.eval(&args[0], env)?;
                let mut acc = self.eval(&args[1], env)?;
                let list = self.eval(&args[2], env)?;

                match list {
                    Value::List(items) => {
                        for item in items.into_iter().rev() {
                            let item_expr = value_to_expr(&item)?;
                            let acc_expr = value_to_expr(&acc)?;
                            acc = self.eval_app(&value_to_expr(&f)?, &[item_expr, acc_expr], env)?;
                        }
                        Ok(acc)
                    }
                    _ => Err(anyhow!("foldr: third argument must be a list")),
                }
            }
            "zip" => {
                if args.len() < 2 {
                    return Err(anyhow!("zip requires 2 arguments"));
                }
                let list1 = self.eval(&args[0], env)?;
                let list2 = self.eval(&args[1], env)?;

                match (list1, list2) {
                    (Value::List(items1), Value::List(items2)) => {
                        let results: Vec<Value> = items1.into_iter()
                            .zip(items2.into_iter())
                            .map(|(a, b)| Value::List(vec![a, b]))
                            .collect();
                        Ok(Value::List(results))
                    }
                    _ => Err(anyhow!("zip: both arguments must be lists")),
                }
            }
            "take" => {
                if args.len() < 2 {
                    return Err(anyhow!("take requires 2 arguments"));
                }
                let n = self.eval(&args[0], env)?;
                let list = self.eval(&args[1], env)?;

                match (n, list) {
                    (Value::Number(n), Value::List(items)) => {
                        let n = n.max(0) as usize;
                        Ok(Value::List(items.into_iter().take(n).collect()))
                    }
                    _ => Err(anyhow!("take: invalid arguments")),
                }
            }
            "drop" => {
                if args.len() < 2 {
                    return Err(anyhow!("drop requires 2 arguments"));
                }
                let n = self.eval(&args[0], env)?;
                let list = self.eval(&args[1], env)?;

                match (n, list) {
                    (Value::Number(n), Value::List(items)) => {
                        let n = n.max(0) as usize;
                        Ok(Value::List(items.into_iter().skip(n).collect()))
                    }
                    _ => Err(anyhow!("drop: invalid arguments")),
                }
            }
            "reverse" => {
                if args.is_empty() {
                    return Err(anyhow!("reverse requires 1 argument"));
                }
                let list = self.eval(&args[0], env)?;

                match list {
                    Value::List(mut items) => {
                        items.reverse();
                        Ok(Value::List(items))
                    }
                    _ => Err(anyhow!("reverse: argument must be a list")),
                }
            }
            "sort" => {
                if args.is_empty() {
                    return Err(anyhow!("sort requires 1 argument"));
                }
                let list = self.eval(&args[0], env)?;

                match list {
                    Value::List(items) => {
                        let mut nums: Vec<i64> = items.iter()
                            .map(|v| match v {
                                Value::Number(n) => Ok(*n),
                                _ => Err(anyhow!("sort: list must contain only numbers")),
                            })
                            .collect::<Result<Vec<_>>>()?;
                        nums.sort();
                        Ok(Value::List(nums.into_iter().map(Value::Number).collect()))
                    }
                    _ => Err(anyhow!("sort: argument must be a list")),
                }
            }
            "length" => {
                if args.is_empty() {
                    return Err(anyhow!("length requires 1 argument"));
                }
                let list = self.eval(&args[0], env)?;

                match list {
                    Value::List(items) => Ok(Value::Number(items.len() as i64)),
                    _ => Err(anyhow!("length: argument must be a list")),
                }
            }
            "head" => {
                if args.is_empty() {
                    return Err(anyhow!("head requires 1 argument"));
                }
                let list = self.eval(&args[0], env)?;

                match list {
                    Value::List(items) => {
                        items.first()
                            .cloned()
                            .ok_or_else(|| anyhow!("head: empty list"))
                    }
                    _ => Err(anyhow!("head: argument must be a list")),
                }
            }
            "tail" => {
                if args.is_empty() {
                    return Err(anyhow!("tail requires 1 argument"));
                }
                let list = self.eval(&args[0], env)?;

                match list {
                    Value::List(items) => {
                        if items.is_empty() {
                            Err(anyhow!("tail: empty list"))
                        } else {
                            Ok(Value::List(items[1..].to_vec()))
                        }
                    }
                    _ => Err(anyhow!("tail: argument must be a list")),
                }
            }
            "sum" => {
                if args.is_empty() {
                    return Err(anyhow!("sum requires 1 argument"));
                }
                let list = self.eval(&args[0], env)?;

                match list {
                    Value::List(items) => {
                        let sum: i64 = items.iter()
                            .map(|v| match v {
                                Value::Number(n) => Ok(*n),
                                _ => Err(anyhow!("sum: list must contain only numbers")),
                            })
                            .collect::<Result<Vec<_>>>()?
                            .into_iter()
                            .sum();
                        Ok(Value::Number(sum))
                    }
                    _ => Err(anyhow!("sum: argument must be a list")),
                }
            }
            "product" => {
                if args.is_empty() {
                    return Err(anyhow!("product requires 1 argument"));
                }
                let list = self.eval(&args[0], env)?;

                match list {
                    Value::List(items) => {
                        let product: i64 = items.iter()
                            .map(|v| match v {
                                Value::Number(n) => Ok(*n),
                                _ => Err(anyhow!("product: list must contain only numbers")),
                            })
                            .collect::<Result<Vec<_>>>()?
                            .into_iter()
                            .product();
                        Ok(Value::Number(product))
                    }
                    _ => Err(anyhow!("product: argument must be a list")),
                }
            }
            "concat" => {
                if args.is_empty() {
                    return Err(anyhow!("concat requires 1 argument"));
                }
                let list = self.eval(&args[0], env)?;

                match list {
                    Value::List(items) => {
                        let mut result = Vec::new();
                        for item in items {
                            match item {
                                Value::List(inner) => result.extend(inner),
                                _ => return Err(anyhow!("concat: must be a list of lists")),
                            }
                        }
                        Ok(Value::List(result))
                    }
                    _ => Err(anyhow!("concat: argument must be a list")),
                }
            }
            "elem" => {
                if args.len() < 2 {
                    return Err(anyhow!("elem requires 2 arguments"));
                }
                let item = self.eval(&args[0], env)?;
                let list = self.eval(&args[1], env)?;

                match list {
                    Value::List(items) => Ok(Value::Bool(items.contains(&item))),
                    _ => Err(anyhow!("elem: second argument must be a list")),
                }
            }
            _ => Err(anyhow!("Unknown builtin: {}", name)),
        }
    }

    fn eval_binop(&mut self, op: BinOp, left: &Expr, right: &Expr, env: &Rc<Env>) -> Result<Value> {
        match op {
            BinOp::PipeForward => {
                // left >> right means right(left)
                let left_val = self.eval(left, env)?;
                let left_expr = value_to_expr(&left_val)?;
                self.eval_app(right, &[left_expr], env)
            }
            BinOp::PipeBackward => {
                // left << right means left(right)
                let right_val = self.eval(right, env)?;
                let right_expr = value_to_expr(&right_val)?;
                self.eval_app(left, &[right_expr], env)
            }
            _ => {
                let left_val = self.eval(left, env)?;
                let right_val = self.eval(right, env)?;

                match op {
                    BinOp::Add => binary_arith(left_val, right_val, |a, b| Ok(a + b)),
                    BinOp::Sub => binary_arith(left_val, right_val, |a, b| Ok(a - b)),
                    BinOp::Mul => binary_arith(left_val, right_val, |a, b| Ok(a * b)),
                    BinOp::Div => binary_arith(left_val, right_val, |a, b| {
                        if b == 0 {
                            Err(anyhow!("Division by zero"))
                        } else {
                            Ok(a / b)
                        }
                    }),
                    BinOp::Mod => binary_arith(left_val, right_val, |a, b| {
                        if b == 0 {
                            Err(anyhow!("Modulo by zero"))
                        } else {
                            Ok(a % b)
                        }
                    }),
                    BinOp::Pow => binary_arith(left_val, right_val, |a, b| {
                        if b < 0 {
                            Err(anyhow!("Negative exponent not supported"))
                        } else {
                            Ok(a.pow(b as u32))
                        }
                    }),
                    BinOp::Eq => Ok(Value::Bool(left_val == right_val)),
                    BinOp::Neq => Ok(Value::Bool(left_val != right_val)),
                    BinOp::Lt => binary_cmp(left_val, right_val, |a, b| a < b),
                    BinOp::Gt => binary_cmp(left_val, right_val, |a, b| a > b),
                    BinOp::Lte => binary_cmp(left_val, right_val, |a, b| a <= b),
                    BinOp::Gte => binary_cmp(left_val, right_val, |a, b| a >= b),
                    BinOp::And => binary_bool(left_val, right_val, |a, b| a && b),
                    BinOp::Or => binary_bool(left_val, right_val, |a, b| a || b),
                    BinOp::Cons => match (left_val, right_val) {
                        (item, Value::List(mut items)) => {
                            items.insert(0, item);
                            Ok(Value::List(items))
                        }
                        _ => Err(anyhow!(":: requires element and list")),
                    }
                    BinOp::Concat => match (left_val, right_val) {
                        (Value::List(mut a), Value::List(b)) => {
                            a.extend(b);
                            Ok(Value::List(a))
                        }
                        (Value::String(mut a), Value::String(b)) => {
                            a.push_str(&b);
                            Ok(Value::String(a))
                        }
                        _ => Err(anyhow!("++ requires two lists or two strings")),
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn eval_list_comp(&mut self, expr: &Expr, var: &str, list_expr: &Expr, guards: &[Expr], env: &Rc<Env>) -> Result<Value> {
        let list_val = self.eval(list_expr, env)?;

        match list_val {
            Value::List(items) => {
                let mut results = Vec::new();

                for item in items {
                    let mut new_env = (**env).clone();
                    new_env.insert(var.to_string(), item);
                    let new_env_rc = Rc::new(new_env);

                    let mut passes = true;
                    for guard in guards {
                        let guard_val = self.eval(guard, &new_env_rc)?;
                        match guard_val {
                            Value::Bool(false) => {
                                passes = false;
                                break;
                            }
                            Value::Bool(true) => {}
                            _ => return Err(anyhow!("Guard must be boolean")),
                        }
                    }

                    if passes {
                        let result = self.eval(expr, &new_env_rc)?;
                        results.push(result);
                    }
                }

                Ok(Value::List(results))
            }
            _ => Err(anyhow!("List comprehension requires a list")),
        }
    }

    fn eval_match(&mut self, expr: &Expr, arms: &[MatchArm], env: &Rc<Env>) -> Result<Value> {
        let val = self.eval(expr, env)?;

        for arm in arms {
            let mut new_env = (**env).clone();
            if self.match_pattern(&arm.pattern, &val, &mut new_env)? {
                return self.eval(&arm.expr, &Rc::new(new_env));
            }
        }

        Err(anyhow!("No pattern matched"))
    }

    fn match_pattern(&self, pattern: &Pattern, value: &Value, env: &mut Env) -> Result<bool> {
        match (pattern, value) {
            (Pattern::Wildcard, _) => Ok(true),
            (Pattern::Var(name), val) => {
                env.insert(name.clone(), val.clone());
                Ok(true)
            }
            (Pattern::Number(n), Value::Number(m)) => Ok(n == m),
            (Pattern::Bool(a), Value::Bool(b)) => Ok(a == b),
            (Pattern::String(a), Value::String(b)) => Ok(a == b),
            (Pattern::List(patterns), Value::List(values)) => {
                if patterns.len() != values.len() {
                    return Ok(false);
                }
                for (p, v) in patterns.iter().zip(values.iter()) {
                    if !self.match_pattern(p, v, env)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            (Pattern::Cons { head, tail }, Value::List(values)) => {
                if values.is_empty() {
                    return Ok(false);
                }
                let head_val = &values[0];
                let tail_val = Value::List(values[1..].to_vec());

                Ok(self.match_pattern(head, head_val, env)? &&
                   self.match_pattern(tail, &tail_val, env)?)
            }
            _ => Ok(false),
        }
    }
}

fn binary_arith<F>(left: Value, right: Value, f: F) -> Result<Value>
where
    F: FnOnce(i64, i64) -> Result<i64>,
{
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(f(a, b)?)),
        _ => Err(anyhow!("Arithmetic operation requires numbers")),
    }
}

fn binary_cmp<F>(left: Value, right: Value, f: F) -> Result<Value>
where
    F: FnOnce(i64, i64) -> bool,
{
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(f(a, b))),
        _ => Err(anyhow!("Comparison requires numbers")),
    }
}

fn binary_bool<F>(left: Value, right: Value, f: F) -> Result<Value>
where
    F: FnOnce(bool, bool) -> bool,
{
    match (left, right) {
        (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(f(a, b))),
        _ => Err(anyhow!("Boolean operation requires booleans")),
    }
}

fn value_to_expr(value: &Value) -> Result<Expr> {
    match value {
        Value::Number(n) => Ok(Expr::Number(*n)),
        Value::Bool(b) => Ok(Expr::Bool(*b)),
        Value::String(s) => Ok(Expr::String(s.clone())),
        Value::List(items) => {
            let exprs: Result<Vec<Expr>> = items.iter().map(value_to_expr).collect();
            Ok(Expr::List(exprs?))
        }
        Value::Function { params, body, env: _ } => Ok(Expr::Lambda {
            params: params.clone(),
            body: Box::new((**body).clone()),
        }),
        Value::Builtin(name) => Ok(Expr::Var(name.clone())),
        _ => Err(anyhow!("Cannot convert value to expression")),
    }
}

pub fn get_builtin_env() -> Env {
    let mut env = Env::new();
    let builtins = vec![
        "map", "filter", "fold", "foldl", "foldr",
        "zip", "take", "drop", "reverse", "sort",
        "length", "head", "tail", "sum", "product",
        "concat", "elem"
    ];

    for name in builtins {
        env.insert(name.to_string(), Value::Builtin(name.to_string()));
    }

    env
}
