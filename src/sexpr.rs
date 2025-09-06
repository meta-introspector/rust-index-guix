use nom::multi::many0_count;
use nom::combinator::{cut, recognize};
use nom::sequence::{pair, terminated};
use nom::bytes::complete::{escaped, tag, take_till, take_while, take_while1};
use nom::character::complete::{anychar, one_of, char, multispace1, none_of};
use nom::{
    IResult,
    sequence::{delimited, preceded},
    multi::many0,
    combinator::map,
    branch::alt,
};

#[derive(Debug, PartialEq)]
pub enum SExpr<'a> {
    Sym(&'a str),
    Str(&'a str),
    List(Vec<SExpr<'a>>),
}

impl<'a> SExpr<'a> {
    pub fn as_list(&self) -> Option<&[SExpr<'a>]> {
        match self {
            SExpr::List(list) => Some(list),
            _ => None
        }
    }

    pub fn if_named(&self, name: &str) -> Option<&[SExpr<'a>]> {
        let l = self.as_list()?;
        if l.get(0)?.as_str()? == name {
            return Some(&l[1..]);
        }
        None
    }

    pub fn into_list(self) -> Option<Vec<SExpr<'a>>> {
        match self {
            SExpr::List(list) => Some(list),
            _ => None
        }
    }

    pub fn as_str(&self) -> Option<&'a str> {
        match self {
            Self::Str(s) => Some(s),
            Self::Sym(s) => Some(s),
            _ => None
        }
    }
}

fn parse_string(input: &str) -> IResult<&str, SExpr<'_>> {
    map(
        delimited(
            char('"'),
            alt((escaped(none_of("\\\""), '\\', anychar), tag(""))),
            cut(char('"'))
        ),
        SExpr::Str,
    )(input)
}

fn ws_or_comment(input: &str) -> IResult<&str, ()> {
    map(many0_count(alt((parse_comment, multispace1))), drop)(input)
}

fn parse_symbol(input: &str) -> IResult<&str, SExpr<'_>> {
    map(recognize(pair(
        take_while1(|c: char| !matches!(c, |')'|'('|'`'|'"'|'\'') && !c.is_ascii_whitespace()),
        take_while(|c: char| !matches!(c, |')'|'('|'`'|'"') && !c.is_ascii_whitespace()),
    )), SExpr::Sym)(input)
}

fn parse_comment(input: &str) -> IResult<&str, &str> {
    preceded(
        tag(";"),
        cut(take_till(|c| c == '\n'))
    )(input)
}

fn parse_list(input: &str) -> IResult<&str, SExpr<'_>> {
    map(
        delimited(
            char('('),
            many0(parse_sexpr),
            preceded(ws_or_comment, cut(char(')'))),
        ),
        SExpr::List,
    )(input)
}

fn parse_quoted(input: &str) -> IResult<&str, SExpr<'_>> {
    preceded(one_of("`'"), parse_sexpr)(input)
}

fn parse_sexpr(input: &str) -> IResult<&str, SExpr<'_>> {
    preceded(ws_or_comment, alt((parse_string, parse_list, parse_quoted, parse_symbol)))(input)
}

fn parse_sexprs(input: &str) -> IResult<&str, Vec<SExpr<'_>>> {
    cut(terminated(many0(parse_sexpr), ws_or_comment))(input)
}

pub fn parse(input: &str) -> Result<Vec<SExpr<'_>>, String> {
    match parse_sexprs(input) {
        Ok((_, parsed)) => Ok(parsed),
        Err(e) => Err(e.to_string()),
    }
}
