use std::collections::HashMap;
use std::fmt;
use crate::lang::ast::*;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    String,
    List(Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Var(usize),
    Unknown,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Bool => write!(f, "Bool"),
            Type::String => write!(f, "String"),
            Type::List(t) => write!(f, "[{}]", t),
            Type::Function(args, ret) => {
                if args.is_empty() {
                    write!(f, "() -> {}", ret)
                } else {
                    write!(f, "{} -> {}",
                        args.iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join(" -> "),
                        ret
                    )
                }
            }
            Type::Var(n) => write!(f, "t{}", n),
            Type::Unknown => write!(f, "?"),
        }
    }
}

pub struct TypeChecker {
    next_var: usize,
    substitutions: HashMap<usize, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            next_var: 0,
            substitutions: HashMap::new(),
        }
    }

    fn fresh_var(&mut self) -> Type {
        let var = Type::Var(self.next_var);
        self.next_var += 1;
        var
    }

    fn apply(&self, ty: &Type) -> Type {
        match ty {
            Type::Var(n) => {
                if let Some(t) = self.substitutions.get(n) {
                    self.apply(t)
                } else {
                    ty.clone()
                }
            }
            Type::List(t) => Type::List(Box::new(self.apply(t))),
            Type::Function(args, ret) => {
                let args = args.iter().map(|t| self.apply(t)).collect();
                Type::Function(args, Box::new(self.apply(ret)))
            }
            _ => ty.clone(),
        }
    }

    fn unify(&mut self, t1: &Type, t2: &Type) -> Result<()> {
        let t1 = self.apply(t1);
        let t2 = self.apply(t2);

        match (&t1, &t2) {
            (Type::Int, Type::Int) => Ok(()),
            (Type::Bool, Type::Bool) => Ok(()),
            (Type::String, Type::String) => Ok(()),
            (Type::List(a), Type::List(b)) => self.unify(a, b),
            (Type::Function(args1, ret1), Type::Function(args2, ret2)) => {
                if args1.len() != args2.len() {
                    return Err(anyhow!("Function arity mismatch"));
                }
                for (a1, a2) in args1.iter().zip(args2.iter()) {
                    self.unify(a1, a2)?;
                }
                self.unify(ret1, ret2)
            }
            (Type::Var(n), t) | (t, Type::Var(n)) => {
                if let Type::Var(m) = t {
                    if n == m {
                        return Ok(());
                    }
                }
                self.substitutions.insert(*n, t.clone());
                Ok(())
            }
            (Type::Unknown, _) | (_, Type::Unknown) => Ok(()),
            _ => Err(anyhow!("Type mismatch: {} vs {}", t1, t2)),
        }
    }

    pub fn infer(&mut self, expr: &Expr, env: &mut HashMap<String, Type>) -> Result<Type> {
        match expr {
            Expr::Number(_) => Ok(Type::Int),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::String(_) => Ok(Type::String),
            Expr::List(items) => {
                if items.is_empty() {
                    Ok(Type::List(Box::new(self.fresh_var())))
                } else {
                    let elem_ty = self.infer(&items[0], env)?;
                    for item in &items[1..] {
                        let ty = self.infer(item, env)?;
                        self.unify(&elem_ty, &ty)?;
                    }
                    Ok(Type::List(Box::new(self.apply(&elem_ty))))
                }
            }
            Expr::Var(name) => {
                env.get(name)
                    .cloned()
                    .ok_or_else(|| anyhow!("Undefined variable: {}", name))
            }
            Expr::Lambda { params, body } => {
                let param_types: Vec<Type> = params.iter().map(|_| self.fresh_var()).collect();

                let mut new_env = env.clone();
                for (param, ty) in params.iter().zip(param_types.iter()) {
                    new_env.insert(param.clone(), ty.clone());
                }

                let ret_ty = self.infer(body, &mut new_env)?;
                Ok(Type::Function(param_types, Box::new(ret_ty)))
            }
            Expr::App { func, args } => {
                let func_ty = self.infer(func, env)?;
                let arg_types: Result<Vec<Type>> = args.iter().map(|arg| self.infer(arg, env)).collect();
                let arg_types = arg_types?;

                let ret_ty = self.fresh_var();
                let expected_func_ty = Type::Function(arg_types, Box::new(ret_ty.clone()));

                self.unify(&func_ty, &expected_func_ty)?;
                Ok(self.apply(&ret_ty))
            }
            Expr::Let { name, value, body } => {
                let value_ty = self.infer(value, env)?;
                let mut new_env = env.clone();
                new_env.insert(name.clone(), value_ty);
                self.infer(body, &mut new_env)
            }
            Expr::If { cond, then_branch, else_branch } => {
                let cond_ty = self.infer(cond, env)?;
                self.unify(&cond_ty, &Type::Bool)?;

                let then_ty = self.infer(then_branch, env)?;
                let else_ty = self.infer(else_branch, env)?;
                self.unify(&then_ty, &else_ty)?;

                Ok(self.apply(&then_ty))
            }
            Expr::BinOp { op, left, right } => {
                let left_ty = self.infer(left, env)?;
                let right_ty = self.infer(right, env)?;

                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod | BinOp::Pow => {
                        self.unify(&left_ty, &Type::Int)?;
                        self.unify(&right_ty, &Type::Int)?;
                        Ok(Type::Int)
                    }
                    BinOp::Eq | BinOp::Neq | BinOp::Lt | BinOp::Gt | BinOp::Lte | BinOp::Gte => {
                        self.unify(&left_ty, &right_ty)?;
                        Ok(Type::Bool)
                    }
                    BinOp::And | BinOp::Or => {
                        self.unify(&left_ty, &Type::Bool)?;
                        self.unify(&right_ty, &Type::Bool)?;
                        Ok(Type::Bool)
                    }
                    BinOp::Cons => {
                        let elem_ty = self.fresh_var();
                        let list_ty = Type::List(Box::new(elem_ty.clone()));
                        self.unify(&left_ty, &elem_ty)?;
                        self.unify(&right_ty, &list_ty)?;
                        Ok(self.apply(&list_ty))
                    }
                    BinOp::Concat => {
                        let elem_ty = self.fresh_var();
                        let list_ty = Type::List(Box::new(elem_ty));
                        self.unify(&left_ty, &list_ty)?;
                        self.unify(&right_ty, &list_ty)?;
                        Ok(self.apply(&list_ty))
                    }
                    BinOp::PipeForward => {
                        // left >> right means right(left)
                        let ret_ty = self.fresh_var();
                        let func_ty = Type::Function(vec![left_ty.clone()], Box::new(ret_ty.clone()));
                        self.unify(&right_ty, &func_ty)?;
                        Ok(self.apply(&ret_ty))
                    }
                    BinOp::PipeBackward => {
                        // left << right means left(right)
                        let ret_ty = self.fresh_var();
                        let func_ty = Type::Function(vec![right_ty.clone()], Box::new(ret_ty.clone()));
                        self.unify(&left_ty, &func_ty)?;
                        Ok(self.apply(&ret_ty))
                    }
                }
            }
            Expr::UnOp { op: UnOp::Neg, expr } => {
                let ty = self.infer(expr, env)?;
                self.unify(&ty, &Type::Int)?;
                Ok(Type::Int)
            }
            Expr::Range { .. } => Ok(Type::List(Box::new(Type::Int))),
            Expr::ListComp { expr, var, list, guards } => {
                let list_ty = self.infer(list, env)?;
                let elem_ty = self.fresh_var();
                self.unify(&list_ty, &Type::List(Box::new(elem_ty.clone())))?;

                let mut new_env = env.clone();
                new_env.insert(var.clone(), self.apply(&elem_ty));

                for guard in guards {
                    let guard_ty = self.infer(guard, &mut new_env)?;
                    self.unify(&guard_ty, &Type::Bool)?;
                }

                let result_elem_ty = self.infer(expr, &mut new_env)?;
                Ok(Type::List(Box::new(result_elem_ty)))
            }
            Expr::Match { expr, arms } => {
                let expr_ty = self.infer(expr, env)?;

                if arms.is_empty() {
                    return Err(anyhow!("Match must have at least one arm"));
                }

                let mut result_ty = None;

                for arm in arms {
                    let mut new_env = env.clone();
                    self.check_pattern(&arm.pattern, &expr_ty, &mut new_env)?;
                    let arm_ty = self.infer(&arm.expr, &mut new_env)?;

                    if let Some(ref ty) = result_ty {
                        self.unify(ty, &arm_ty)?;
                    } else {
                        result_ty = Some(arm_ty);
                    }
                }

                Ok(self.apply(result_ty.as_ref().unwrap()))
            }
        }
    }

    fn check_pattern(&mut self, pattern: &Pattern, ty: &Type, env: &mut HashMap<String, Type>) -> Result<()> {
        match pattern {
            Pattern::Wildcard => Ok(()),
            Pattern::Var(name) => {
                env.insert(name.clone(), ty.clone());
                Ok(())
            }
            Pattern::Number(_) => self.unify(ty, &Type::Int),
            Pattern::Bool(_) => self.unify(ty, &Type::Bool),
            Pattern::String(_) => self.unify(ty, &Type::String),
            Pattern::List(patterns) => {
                let elem_ty = self.fresh_var();
                self.unify(ty, &Type::List(Box::new(elem_ty.clone())))?;
                for p in patterns {
                    self.check_pattern(p, &self.apply(&elem_ty), env)?;
                }
                Ok(())
            }
            Pattern::Cons { head, tail } => {
                let elem_ty = self.fresh_var();
                let list_ty = Type::List(Box::new(elem_ty.clone()));
                self.unify(ty, &list_ty)?;

                self.check_pattern(head, &self.apply(&elem_ty), env)?;
                self.check_pattern(tail, &self.apply(&list_ty), env)?;
                Ok(())
            }
        }
    }
}

pub fn get_builtin_env() -> HashMap<String, Type> {
    let mut env = HashMap::new();

    let a = Type::Var(1000);
    let b = Type::Var(1001);

    // map :: (a -> b) -> [a] -> [b]
    env.insert("map".to_string(),
        Type::Function(
            vec![
                Type::Function(vec![a.clone()], Box::new(b.clone())),
                Type::List(Box::new(a.clone()))
            ],
            Box::new(Type::List(Box::new(b.clone())))
        )
    );

    // filter :: (a -> Bool) -> [a] -> [a]
    env.insert("filter".to_string(),
        Type::Function(
            vec![
                Type::Function(vec![a.clone()], Box::new(Type::Bool)),
                Type::List(Box::new(a.clone()))
            ],
            Box::new(Type::List(Box::new(a.clone())))
        )
    );

    // fold/foldl/foldr :: (b -> a -> b) -> b -> [a] -> b
    for name in &["fold", "foldl", "foldr"] {
        env.insert(name.to_string(),
            Type::Function(
                vec![
                    Type::Function(vec![b.clone(), a.clone()], Box::new(b.clone())),
                    b.clone(),
                    Type::List(Box::new(a.clone()))
                ],
                Box::new(b.clone())
            )
        );
    }

    // zip :: [a] -> [b] -> [(a, b)]
    env.insert("zip".to_string(),
        Type::Function(
            vec![
                Type::List(Box::new(a.clone())),
                Type::List(Box::new(b.clone()))
            ],
            Box::new(Type::List(Box::new(a.clone()))) // Simplified - we don't have tuples
        )
    );

    // take :: Int -> [a] -> [a]
    env.insert("take".to_string(),
        Type::Function(
            vec![Type::Int, Type::List(Box::new(a.clone()))],
            Box::new(Type::List(Box::new(a.clone())))
        )
    );

    // drop :: Int -> [a] -> [a]
    env.insert("drop".to_string(),
        Type::Function(
            vec![Type::Int, Type::List(Box::new(a.clone()))],
            Box::new(Type::List(Box::new(a.clone())))
        )
    );

    // reverse :: [a] -> [a]
    env.insert("reverse".to_string(),
        Type::Function(
            vec![Type::List(Box::new(a.clone()))],
            Box::new(Type::List(Box::new(a.clone())))
        )
    );

    // sort :: [Int] -> [Int]
    env.insert("sort".to_string(),
        Type::Function(
            vec![Type::List(Box::new(Type::Int))],
            Box::new(Type::List(Box::new(Type::Int)))
        )
    );

    // length :: [a] -> Int
    env.insert("length".to_string(),
        Type::Function(
            vec![Type::List(Box::new(a.clone()))],
            Box::new(Type::Int)
        )
    );

    // head :: [a] -> a
    env.insert("head".to_string(),
        Type::Function(
            vec![Type::List(Box::new(a.clone()))],
            Box::new(a.clone())
        )
    );

    // tail :: [a] -> [a]
    env.insert("tail".to_string(),
        Type::Function(
            vec![Type::List(Box::new(a.clone()))],
            Box::new(Type::List(Box::new(a.clone())))
        )
    );

    // sum :: [Int] -> Int
    env.insert("sum".to_string(),
        Type::Function(
            vec![Type::List(Box::new(Type::Int))],
            Box::new(Type::Int)
        )
    );

    // product :: [Int] -> Int
    env.insert("product".to_string(),
        Type::Function(
            vec![Type::List(Box::new(Type::Int))],
            Box::new(Type::Int)
        )
    );

    // concat :: [[a]] -> [a]
    env.insert("concat".to_string(),
        Type::Function(
            vec![Type::List(Box::new(Type::List(Box::new(a.clone()))))],
            Box::new(Type::List(Box::new(a.clone())))
        )
    );

    // elem :: a -> [a] -> Bool
    env.insert("elem".to_string(),
        Type::Function(
            vec![a.clone(), Type::List(Box::new(a.clone()))],
            Box::new(Type::Bool)
        )
    );

    env
}
