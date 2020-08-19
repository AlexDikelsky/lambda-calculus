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


#[test]
fn is_id_norm() {
    assert!(id().is_normal_form() == true);
}

#[test]
fn is_fls_norm() {
    assert!(fls().is_normal_form() == true);
}

#[test]
fn is_tru_norm() {
    assert!(tru().is_normal_form() == true);
}

#[test]
fn is_and_norm() {
    assert!(and().is_normal_form() == true);
}

#[test]
fn is_abs_abs_norm() {
    assert!(apply(id(), id()).is_normal_form()
            == false);
}

#[test]
fn is_abs_var_norm() {
    assert!(apply(id(), Var('x')).is_normal_form()
            == false);
}

#[test]
fn b_normal() {
    let x = 
      apply(
        abstraction(
            'x', apply(Var('x'), Var('y'))),
        abstraction(
            'u', apply(apply(Var('v'), Var('u')), Var('u'))),
    );
    assert!(x.is_normal_form() == false);
}
