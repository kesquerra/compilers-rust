pub enum UnaryOperand {
    USub,
}

pub enum BinaryOperand {
    Add,
    Sub
}

pub enum Expr {
    Constant(u64),
    UnaryOp(UnaryOperand, Box<Expr>),
    BinOp(Box<Expr>, BinaryOperand, Box<Expr>),
    Call(Fn, [Box<Expr>; 1]),
}

pub enum Fn {
    Name(String)
}

pub enum Stmt {
    Expr(Expr),
}

pub struct Module(pub Vec<Stmt>);