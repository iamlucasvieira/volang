use crate::parse::*;

pub fn eval(expr: Expr) {
    match expr {
        Expr::Call(name, arg) => {
            if name == "echo" {
                println!("{arg}");
            }
        }
    }
}