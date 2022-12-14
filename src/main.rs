mod ast;
mod math;
mod eval;
use ast::Expr::*;
use ast::{Expr, Stmt, Module, BinaryOperand::Add};
use eval::eval_prog;

fn main() {
    let ast1: Expr = BinOp(Box::new(Constant(8)), Add, Box::new(Constant(10)));
    let prog = Module(vec!{Stmt::Expr(ast1)});
    let eval = eval_prog(prog);
    println!("{:?}", eval)
}

fn is_exp(e:Expr) -> bool {
    match e {
        Constant(_) => true,
        UnaryOp(_, e) => is_exp(*e),
        BinOp(e1, _, e2) => is_exp(*e1) && is_exp(*e2),
        Call(..) => true
    }
}

fn stmt(s:Stmt) -> bool {
    match s {
        Stmt::Expr(Call(_, [e])) => is_exp(*e),
        Stmt::Expr(e) => is_exp(e)
    }
}

fn is_lint(p:Module) -> bool {
    p.0.into_iter().all(stmt)
}
