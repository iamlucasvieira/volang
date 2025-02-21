mod eval;
mod parse;

fn main() {
    let input = include_str!("../input.vo");
    let (_, expr) = parse::parse_call(input).unwrap();
    eval::eval(expr);
}
