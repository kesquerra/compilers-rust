#[derive(Debug)]
pub enum UnaryOperand {
    USub,
}

#[derive(Debug)]
pub enum BinaryOperand {
    Add,
    Sub
}


#[derive(Debug)]
pub enum Expr {
    Constant(i64),
    UnaryOp(UnaryOperand, Box<Expr>),
    BinOp(Box<Expr>, BinaryOperand, Box<Expr>),
    Call(Fn, [Box<Expr>; 1]),
}

#[derive(Debug)]
pub enum Fn {
    Name(String)
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Debug)]
pub struct Module(pub Vec<Stmt>);