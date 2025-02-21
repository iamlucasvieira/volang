use nom::bytes::{complete::tag, take_until};
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::{IResult, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Atom {
    String(String),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(string) => write!(f, "{string}"),
        }
    }
}

pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    map(parser, |string: &str| Atom::String(string.to_string())).parse(input)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Call(String, Atom),
}

pub fn parse_call(input: &str) -> IResult<&str, Expr> {
    let parse_name = alpha1;
    let parse_arg = delimited(tag("("), parse_string, tag(")"));
    let parser = (parse_name, parse_arg);
    map(parser, |(name, arg)| Expr::Call(name.to_string(), arg)).parse(input)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let input = "\"Hello, world!\"";
        let (_, result) = parse_string(input).unwrap();
        assert_eq!(result, Atom::String("Hello, world!".to_string()));
    }

    #[test]
    fn test_parse_call() {
        let input = "echo(\"Hello, world!\")";
        let (_, result) = parse_call(input).unwrap();
        assert_eq!(result, Expr::Call("echo".to_string(), Atom::String("Hello, world!".to_string())));
    }
}

