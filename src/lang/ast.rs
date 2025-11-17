use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals
    Number(i64),
    Bool(bool),
    String(String),
    List(Vec<Expr>),

    // Variables and functions
    Var(String),
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    App {
        func: Box<Expr>,
        args: Vec<Expr>,
    },

    // Let binding
    Let {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },

    // Pattern matching
    Match {
        expr: Box<Expr>,
        arms: Vec<MatchArm>,
    },

    // Conditionals
    If {
        cond: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },

    // Binary operations
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    // Unary operations
    UnOp {
        op: UnOp,
        expr: Box<Expr>,
    },

    // Range
    Range {
        start: i64,
        end: i64,
    },

    // List comprehension
    ListComp {
        expr: Box<Expr>,
        var: String,
        list: Box<Expr>,
        guards: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard,
    Var(String),
    Number(i64),
    Bool(bool),
    String(String),
    List(Vec<Pattern>),
    Cons {
        head: Box<Pattern>,
        tail: Box<Pattern>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    // Comparison
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,

    // Logical
    And,
    Or,

    // List operations
    Cons,
    Concat,

    // Composition
    PipeForward,
    PipeBackward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Expr::Var(name) => write!(f, "{}", name),
            Expr::Lambda { params, .. } => {
                write!(f, "\\{} -> ...", params.join(", "))
            }
            _ => write!(f, "<expr>"),
        }
    }
}
