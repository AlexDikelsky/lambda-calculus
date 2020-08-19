use crate::terms::Term;
use crate::terms::Term::App;
use crate::terms::Term::Abs;
use crate::lambda::TermParser;
use crate::terms;

pub fn apply(a: Term, b: Term) -> Term {
    App(Box::new(a), Box::new(b))
}

pub fn abstraction(c: char, b: Term) -> Term {
    Abs(c, Box::new(b))
}


pub fn parse(s: &str) -> Result<terms::Term, lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>, &'static str>> {
    TermParser::new().parse(s)
}
