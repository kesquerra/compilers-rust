use crate::ast::{Expr, Expr::*, UnaryOperand::USub, BinaryOperand::*};

pub fn calc(e: Expr) -> Expr {
    match e {
        BinOp(l, op, r) => {
            match (*l, *r) {
                (e1, Constant(0)) | (Constant(0), e1) => e1,
                (Constant(n1), Constant(n2)) => {
                    match op {
                        Add => Constant(n1+n2),
                        Sub => Constant(n1-n2)
                    }
                },
                (e1, UnaryOp(USub, e2)) => {
                    match op {
                        Add => calc(BinOp(Box::new(e1), Sub, e2)),
                        Sub => calc(BinOp(Box::new(e1), Add, e2))
                    }
                },
                (a, b) => BinOp(Box::new(a), op, Box::new(b))
            }
        },
        _ => e
    }
}

pub fn negate(e: Expr) -> Expr {
    match e {
        Constant(n) => Constant(-n),
        _ => UnaryOp(USub, Box::new(e))
    }
}