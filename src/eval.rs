use std::collections::HashMap;

use crate::parse::*;

pub fn eval(expr: Expr, context: &mut HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Call(name, arg) => {
            if name == "echo" {
                let arg = eval(*arg, context);
                println!("{arg:?}");
            }
            Expr::Void
        }
        Expr::Let(name, value) => {
            context.insert(name, *value);
            Expr::Void
        }
        Expr::Constant(ref atom) => match atom {
            Atom::Name(name) => context.get(name).unwrap().clone(),
            _ => expr,
        },
        Expr::Void => expr,
    }
}

