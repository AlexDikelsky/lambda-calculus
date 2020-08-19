#![allow(unused_imports)]
use crate::terms::Term::Var;
use crate::terms::Term;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::combinators::one_arg_combinators::id;
use crate::combinators::bool_combinators::tru;
use crate::combinators::bool_combinators::fls;
use crate::combinators::bool_combinators::not;

use crate::aux::apply;
use crate::aux::abstraction;

#[test]
fn not_tru() {
    let a = apply(not(), tru()).to_normal_form();
    dbg!(&a);
    assert!(a == fls());
        
}

#[test]
fn not_fls() {
    let a = apply(not(), fls()).to_normal_form();
    assert!(
        a == tru());
}
