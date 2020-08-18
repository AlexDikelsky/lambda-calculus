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
use crate::combinators::and;
use crate::combinators::or;
use crate::combinators::xor;

use crate::aux::apply;
use crate::aux::abstraction;


#[test]
fn fls_xor_fls() {
    let flsfls = apply(apply(xor(), fls()), fls());
    let norm = flsfls.to_normal_form();
    dbg!(&norm);
    assert!(norm == tru());
}
#[test]
fn fls_xor_tru() {
    let flstru = apply(apply(xor(), fls()), tru());
    let norm = flstru.to_normal_form();
    dbg!(&norm);
    assert!(norm == fls());
}
#[test]
fn tru_xor_fls() {
    let trufls = apply(apply(xor(), tru()), fls());
    let norm = trufls.to_normal_form();
    dbg!(&norm);
    assert!(norm == fls());
}
#[test]
fn tru_xor_tru() {
    let trutru = apply(apply(xor(), tru()), tru());
    let norm = trutru.to_normal_form();
    dbg!(&norm);
    assert!(norm == tru());
}
