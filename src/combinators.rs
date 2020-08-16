use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::terms::Term;

pub fn id() -> Box<Term> {
    Box::new(Abs('x', Box::new(Var('x'))))
}

pub fn tru() -> Box<Term> {
    Box::new(Abs('y', Box::new(Abs('z', Box::new(Var('y'))))))
}

pub fn fls() -> Box<Term> {
    Box::new(Abs('a', Box::new(Abs('b', Box::new(Var('b'))))))
}
