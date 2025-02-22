use nom::branch::alt;
use nom::bytes::{complete::tag, take_until};
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::{error::ParseError, IResult, Parser};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Atom {
    String(String),
    Name(String),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(string) => write!(f, "{string}"),
            Atom::Name(string) => write!(f, "{string}"),
        }
    }
}

pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    map(parser, |string: &str| Atom::String(string.to_string())).parse(input)
}

pub fn parse_name(input: &str) -> IResult<&str, Atom> {
    map(alpha1, |string: &str| Atom::Name(string.to_string())).parse(input)
}

pub fn parse_atom(input: &str) -> IResult<&str, Atom> {
    alt((parse_string, parse_name)).parse(input)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Void,
    Constant(Atom),
    Let(String, Box<Expr>),
    Call(String, Box<Expr>),
}

pub fn parse_constant(input: &str) -> IResult<&str, Expr> {
    map(parse_atom, Expr::Constant).parse(input)
}

pub fn parse_let(input: &str) -> IResult<&str, Expr> {
    let parse_name = preceded(tag("vo"), ws(alpha1));
    let parse_equals = preceded(tag("="), ws(parse_expr));
    let parser = (parse_name, parse_equals);
    map(parser, |(name, equals)| {
        Expr::Let(name.to_string(), Box::new(equals))
    })
    .parse(input)
}

pub fn parse_call(input: &str) -> IResult<&str, Expr> {
    let parse_name = alpha1;
    let parse_arg = delimited(tag("("), parse_expr, tag(")"));
    let parser = (parse_name, parse_arg);
    map(parser, |(name, arg)| {
        Expr::Call(name.to_string(), Box::new(arg))
    })
    .parse(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_let, parse_call, parse_constant)).parse(input)
}

pub fn parser(input: &str) -> IResult<&str, Vec<Expr>> {
    many0(ws(parse_expr)).parse(input)
}

pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
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
    fn test_parse_name() {
        let input = "test";
        let (_, result) = parse_name(input).unwrap();
        assert_eq!(result, Atom::Name("test".to_string()));
    }

    #[test]
    fn test_parse_call() {
        let input = "echo(\"Hello, world!\")";
        let (_, result) = parse_call(input).unwrap();
        assert_eq!(
            result,
            Expr::Call(
                "echo".to_string(),
                Box::new(Expr::Constant(Atom::String("Hello, world!".to_string())))
            )
        );
    }

    #[test]
    fn test_parse_let() {
        let test_cases = vec!["vo a = \"test\"", "vo   a    =    \"test\"   "];
        for tc in test_cases {
            let (_, result) = parse_let(tc).unwrap();
            assert_eq!(
                result,
                Expr::Let(
                    "a".to_string(),
                    Box::new(Expr::Constant(Atom::String("test".to_string())))
                )
            )
        }
    }
}
