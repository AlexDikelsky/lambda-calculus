#![allow(unused_imports)]
use crate::terms::Term::Var;
use crate::terms::Term;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::combinators::id;
use crate::combinators::tru;
use crate::combinators::fls;
use crate::combinators::not;

use crate::aux::apply;
use crate::aux::abstraction;

#[test]
fn not_tru() {
    assert!(
        apply(not(), tru()).to_normal_form() == fls());
}

#[test]
fn not_fls() {
    assert!(
        apply(not(), fls()).to_normal_form() == tru());
}
