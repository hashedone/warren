use crate::ast::{Statement, Term};
use nom::{
    branch::alt,
    bytes::complete::{take_while, take_while1},
    character::complete::{char, multispace0 as ws},
    combinator::map,
    multi::separated_nonempty_list,
    sequence::{delimited, terminated, tuple},
};

type IResult<I, O> = nom::IResult<I, O, nom::error::VerboseError<I>>;

fn ident(s: &str) -> IResult<&str, String> {
    let head_pred = |c: char| (c.is_alphabetic() || c == '_');
    let tail_pred = |c: char| (c.is_alphanumeric() || c == '_');

    map(
        tuple((take_while1(head_pred), take_while(tail_pred))),
        |(h, t)| format!("{}{}", h, t),
    )(s)
}

fn constant(s: &str) -> IResult<&str, Term> {
    map(ident, Term::Const)(s)
}

fn variable(s: &str) -> IResult<&str, Term> {
    map(tuple((char('?'), ident)), |(_, c)| Term::Var(c))(s)
}

fn structure(s: &str) -> IResult<&str, Term> {
    map(
        tuple((
            ident,
            ws,
            char('('),
            separated_nonempty_list(char(','), delimited(ws, term, ws)),
            char(')'),
        )),
        |(name, _, _, subterms, _)| Term::Struct(name, subterms),
    )(s)
}

fn term(s: &str) -> IResult<&str, Term> {
    alt((structure, variable, constant))(s)
}

fn query(s: &str) -> IResult<&str, Statement> {
    map(terminated(term, char('?')), Statement::Query)(s)
}

pub fn statement(s: &str) -> IResult<&str, Statement> {
    query(s)
}

pub fn parse(s: &str) -> Result<Statement, nom::Err<nom::error::VerboseError<&str>>> {
    statement(s).map(|(_, r)| r)
}
