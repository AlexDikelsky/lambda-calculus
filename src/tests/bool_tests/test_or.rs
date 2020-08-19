#![allow(unused_imports)]
use crate::terms::Term::Var;
use crate::terms::Term;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::combinators::one_arg_combinators::id;
use crate::combinators::bool_combinators::tru;
use crate::combinators::bool_combinators::fls;
use crate::combinators::bool_combinators::or;

use crate::aux::apply;
use crate::aux::abstraction;

#[test]
fn fls_or_fls() {
    let flsfls = apply(apply(or(), fls()), fls());
    let norm = flsfls.to_normal_form();
    dbg!(&norm);
    assert!(norm == fls());
}
#[test]
fn fls_or_tru() {
    let flstru = apply(apply(or(), fls()), tru());
    let norm = flstru.to_normal_form();
    dbg!(&norm);
    assert!(norm == tru());
}
#[test]
fn tru_or_fls() {
    let trufls = apply(apply(or(), tru()), fls());
    let norm = trufls.to_normal_form();
    dbg!(&norm);
    assert!(norm == tru());
}
#[test]
fn tru_or_tru() {
    let trutru = apply(apply(or(), tru()), tru());
    let norm = trutru.to_normal_form();
    dbg!(&norm);
    assert!(norm == tru());
}
