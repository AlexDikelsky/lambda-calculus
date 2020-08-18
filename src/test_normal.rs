#![allow(unused_imports)]
use crate::terms::Term::Var;
use crate::terms::Term;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::combinators::id;
use crate::combinators::tru;
use crate::combinators::fls;
use crate::combinators::and;
use crate::constants::var_a;
use crate::constants::var_b;
use crate::constants::var_u;
use crate::constants::var_v;
use crate::constants::var_w;
use crate::constants::var_x;
use crate::constants::var_y;
use crate::constants::var_z;

fn apply(a: Term, b: Term) -> Box<Term> {
    Box::new(App(Box::new(a), Box::new(b)))
}

fn abstraction(c: char, b: Term) -> Box<Term> {
    Box::new(Abs(c, Box::new(b)))
}

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
    assert!(apply(*id(), *id()).is_normal_form()
            == false);
}

#[test]
fn is_abs_var_norm() {
    assert!(apply(*id(), *var_x()).is_normal_form()
            == false);
}


