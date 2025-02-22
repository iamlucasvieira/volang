use std::collections::HashMap;

use crate::parse::*;

pub fn eval(expr: Expr, context: &mut HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Call(name, args) => {
            if name == "echo" {
                for arg in args {
                    let arg = eval(arg, context);
                    print!("{arg:?}");
                }
            } else {
                match context.get(&name) {
                    Some(Expr::Closure(parameters, body)) => {
                        let mut scope = context.clone();
                        for (parameter, arg) in parameters.iter().zip(args.into_iter()) {
                            let expr = eval(arg, &mut scope);
                            scope.insert(parameter.clone(), expr);
                        }

                        for expr in body {
                            eval(expr.clone(), &mut scope);
                        }
                    }
                    _ => panic!("Function `{name}` doesn't exist"),
                }
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
        Expr::Void | Expr::Closure(_, _) => expr,
    }
}

