mod ast;
use ast::Expr::*;
use ast::{Expr, Stmt, Module, BinaryOperand::Add};

fn main() {
    let ast1: Expr = BinOp(Box::new(Constant(8)), Add, Box::new(Constant(10)));
    let valid: bool = is_Lint(Module(vec!{Stmt::Expr(ast1)}));
    println!("{}", valid);
}

fn is_exp(e:Expr) -> bool {
    match e {
        Expr::Constant(_) => true,
        Expr::UnaryOp(_, e) => is_exp(*e),
        Expr::BinOp(e1, _, e2) => is_exp(*e1) && is_exp(*e2),
        Expr::Call(..) => true
    }
}

fn stmt(s:Stmt) -> bool {
    match s {
        Stmt::Expr(Expr::Call(_, [e])) => is_exp(*e),
        Stmt::Expr(e) => is_exp(e)
    }
}

fn is_Lint(p:Module) -> bool {
    p.0.into_iter().all(stmt)
}
