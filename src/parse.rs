use nom::branch::alt;
use nom::bytes::{complete::tag, take_until};
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::map;
use nom::multi::{many0, separated_list0};
use nom::number::complete::double;
use nom::sequence::{delimited, preceded};
use nom::{error::ParseError, IResult, Parser};

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    String(String),
    Name(String),
    Float(f64),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(string) => write!(f, "\"{string}\""),
            Atom::Float(number) => write!(f, "{number}"),
            Atom::Name(string) => write!(f, "{string}"),
        }
    }
}

pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    map(parser, |string: &str| Atom::String(string.to_string())).parse(input)
}

pub fn parse_float(input: &str) -> IResult<&str, Atom> {
    map(double, |number: f64| Atom::Float(number)).parse(input)
}

pub fn parse_name(input: &str) -> IResult<&str, Atom> {
    map(alpha1, |string: &str| Atom::Name(string.to_string())).parse(input)
}

pub fn parse_atom(input: &str) -> IResult<&str, Atom> {
    alt((parse_float, parse_string, parse_name)).parse(input)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Void,
    Constant(Atom),
    Let(String, Box<Expr>),
    Call(String, Vec<Expr>),
    Closure(Vec<String>, Vec<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Constant(atom) => write!(f, "{atom}"),
            _ => write!(f, ""),
        }
    }
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
    let parse_args = delimited(
        tag("("),
        separated_list0(tag(","), ws(parse_expr)),
        tag(")"),
    );
    let parser = (parse_name, parse_args);
    map(parser, |(name, args)| Expr::Call(name.to_string(), args)).parse(input)
}

pub fn parse_closure(input: &str) -> IResult<&str, Expr> {
    let parse_name = map(alpha1, String::from);
    let parse_args = delimited(
        tag("|"),
        separated_list0(tag(","), ws(parse_name)),
        tag("|"),
    );
    let parser = (ws(parse_args), ws(parse_expr));
    map(parser, |(args, expr)| Expr::Closure(args, vec![expr])).parse(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_let, parse_call, parse_closure, parse_constant)).parse(input)
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
    fn test_parse_float() {
        let input = "1.0";
        let (_, result) = parse_float(input).unwrap();
        assert_eq!(result, Atom::Float(1.0));
    }

    #[test]
    fn test_parse_call() {
        let input = "echo(\"Hello, world!\")";
        let (_, result) = parse_call(input).unwrap();
        assert_eq!(
            result,
            Expr::Call(
                "echo".to_string(),
                vec![Expr::Constant(Atom::String("Hello, world!".to_string()))]
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

    #[test]
    fn test_parse_closure() {
        struct TestCase {
            input: String,
            expected: Expr,
        }

        let test_cases = vec![
            TestCase {
                input: "|a| \"hello\"".to_string(),
                expected: Expr::Closure(
                    vec!["a".to_string()],
                    vec![Expr::Constant(Atom::String("hello".to_string()))],
                ),
            },
            TestCase {
                input: "|a, b| \"hello\"".to_string(),
                expected: Expr::Closure(
                    vec!["a".to_string(), "b".to_string()],
                    vec![Expr::Constant(Atom::String("hello".to_string()))],
                ),
            },
        ];

        for tc in test_cases {
            let (_, result) = parse_closure(&tc.input).unwrap();
            assert_eq!(result, tc.expected);
        }
    }
}
