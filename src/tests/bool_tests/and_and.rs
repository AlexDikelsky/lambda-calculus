#![allow(unused_imports)]
use crate::terms::Term::Var;
use crate::terms::Term;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::combinators::one_arg_combinators::id;
use crate::combinators::bool_combinators::tru;
use crate::combinators::bool_combinators::fls;
use crate::combinators::bool_combinators::and;

use crate::aux::apply;
use crate::aux::abstraction;

//fn fls_and_fls() -> Term {
//    apply(apply(and(), fls()), fls())
//}
//fn fls_and_tru() -> Term {
//    apply(apply(and(), fls()), tru())
//}
//fn tru_and_fls() -> Term {
//    apply(apply(and(), tru()), fls())
//}
//fn tru_and_tru() -> Term {
//    apply(apply(and(), tru()), tru())
//}
//
//fn ands(n: usize, op: Term) -> Term {
//    (0..n).fold(op.clone(), |a, b| apply(a, op.clone()))
//}
//
//#[test]
//fn ntrutru() {
//    //assert!(ands(4, tru_and_tru()).to_normal_form() == ands(10, tru_and_tru()).to_normal_form());
//    //assert!(apply(tru_and_tru(), tru_and_tru()).to_normal_form() == tru_and_tru().to_normal_form());
//
//
//    let s = apply(apply(and(), tru_and_tru()), tru_and_tru()).to_normal_form();
//    dbg!(&s);
//    
//    assert!(s == tru());
//}
//
//#[test]
//fn ntrufls() {
//    assert!(ands(4, tru_and_fls()).to_normal_form() == ands(10, tru_and_fls()).to_normal_form());
//    assert!(ands(5, tru_and_fls()).to_normal_form() == tru_and_fls().to_normal_form());
//}
//#[test]
//fn nflstru() {
//    assert!(ands(4, fls_and_tru()).to_normal_form() == ands(10, fls_and_tru()).to_normal_form());
//    assert!(ands(5, fls_and_tru()).to_normal_form() == fls_and_tru().to_normal_form());
//}
//#[test]
//fn nflsfls() {
//    assert!(ands(4, fls_and_fls()).to_normal_form() == ands(10, fls_and_fls()).to_normal_form());
//    assert!(ands(5, fls_and_fls()).to_normal_form() == fls_and_fls().to_normal_form());
//}
