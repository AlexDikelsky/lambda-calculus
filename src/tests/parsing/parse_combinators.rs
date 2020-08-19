#![allow(unused_imports)]
use crate::combinators::tru;
use crate::combinators::and;
use crate::combinators::fls;
use crate::combinators::omega;
use crate::combinators::omega_parts;
use crate::combinators::id;
use crate::aux::apply;
use crate::aux::abstraction;
use crate::terms;
use crate::lalrpop_util;

use crate::lambda::TermParser;

fn parse(s: &str) -> Result<terms::Term, lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>, &'static str>> {
    TermParser::new().parse(s)
}

#[test]
fn identity() {
    let parsed = TermParser::new().parse("(λx.x)");

    dbg!(&parsed);
    assert!(parsed.unwrap() == id());
}

#[test]
fn test_omega() {
    let parsed_half = parse("(λx.(xx))");

    
    assert!(parsed_half.unwrap() == omega_parts());

    let parsed_full = parse("((λx.(xx))(λx.(xx)))");

    dbg!(&parsed_full);
    dbg!(omega());

    assert!(parsed_full.unwrap() == omega());

    assert!(false);
}

