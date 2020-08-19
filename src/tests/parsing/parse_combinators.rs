#![allow(unused_imports)]
use crate::combinators::bool_combinators::tru;
use crate::combinators::bool_combinators::and;
use crate::combinators::bool_combinators::fls;
use crate::combinators::endless_combinators::omega;
use crate::combinators::endless_combinators::omega_parts;
use crate::combinators::one_arg_combinators::id;
use crate::aux::apply;
use crate::aux::abstraction;
use crate::terms;

use crate::aux::parse;

#[test]
fn identity() {
    let parsed = parse("(λx.x)");

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

}

#[test]
fn bools() {
    let ltrue = parse("(λx.(λy.x))");
    assert!(ltrue.unwrap() == tru());

    let lfalse = parse("(λx.(λy.y))");
    assert!(lfalse.unwrap() == fls());
}
    
