use crate::{ast::{Expr, Expr::*, UnaryOperand::USub, Stmt, Module}, math::{calc, negate}};

pub fn eval_expr(e:Expr) -> Expr {
    match e {
        BinOp(l, op, r) => {
            let left = eval_expr(*l);
            let right = eval_expr(*r);
            calc(BinOp(Box::new(left), op, Box::new(right)))
        }
        UnaryOp(USub, v) => negate(*v),
        Constant(_) => e,
        Call(_, _) => e
    }
}

pub fn eval_stmt(s:Stmt) -> Stmt {
    match s {
        Stmt::Expr(Call(name, [arg])) => Stmt::Expr(Call(name, [Box::new(eval_expr(*arg))])),
        Stmt::Expr(e) => Stmt::Expr(eval_expr(e))
    
    }
}

pub fn eval_prog(p:Module) -> Module {
    match p {
        Module(body) => {
            let new_body: Vec<Stmt> = body.into_iter().map(eval_stmt).collect();
            Module(new_body)
        }
    }
}