use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::terms::Term;

pub fn var_a() -> Box<Term> {
    Box::new(Var('a'))
}

pub fn var_b() -> Box<Term> {
    Box::new(Var('b'))
}
