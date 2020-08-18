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
