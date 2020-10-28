#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    Assignment(String, Expr),
    Return(Option<Expr>),
    Conditional(Expr, Expr, Option<Expr>),
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Constant(f64),
    Binary(Box<Expr>, Op, Box<Expr>),
    Unary(Op, Box<Expr>),
    Call(String, Vec<Box<Expr>>),
    Symbol(String),
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub name: String,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<FuncDecl>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}
