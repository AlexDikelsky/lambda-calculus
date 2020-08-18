use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::terms::Term;

fn gen(c: char) -> Box<Term> {
    Box::new(Var(c))
}

pub fn var_a() -> Box<Term> {
    gen('a')
}

pub fn var_b() -> Box<Term> {
    gen('b')
}

pub fn var_c() -> Box<Term> {
    gen('c')
} 

pub fn var_u() -> Box<Term> {
    gen('u')
}
pub fn var_v() -> Box<Term> {
    gen('v')
}
pub fn var_w() -> Box<Term> {
    gen('w')
}

pub fn var_x() -> Box<Term> {
    gen('x')
}

pub fn var_y() -> Box<Term> {
    gen('y')
}
pub fn var_z() -> Box<Term> {
    gen('z')
}
