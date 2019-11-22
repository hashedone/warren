use crate::ast::{Directive, Statement, Term};
use nom::{
    branch::alt,
    bytes::complete::{take_while, take_while1, tag},
    character::complete::{char, multispace0 as ws},
    combinator::map,
    multi::separated_nonempty_list,
    sequence::{delimited, terminated, tuple, preceded},
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

fn fact(s: &str) -> IResult<&str, Statement> {
    map(terminated(term, char('.')), Statement::Fact)(s)
}

pub fn statement(s: &str) -> IResult<&str, Directive> {
    map(alt((query, fact)), Directive::Statement)(s)
}

pub fn assembly(s: &str) -> IResult<&str, Directive> {
    map(
        preceded(
            tuple((tag("@asm"), ws)),
            alt((query, fact))
        ),
        Directive::Assembly
    )(s)
}


pub fn directive(s: &str) -> IResult<&str, Directive> {
    alt((statement, assembly))(s)
}

pub fn parse(s: &str) -> Result<
    Directive,
    nom::Err<nom::error::VerboseError<&str>>
> {
    directive(s).map(|(_, r)| r)
}
