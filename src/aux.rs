use crate::terms::Term;
use crate::terms::Term::App;
use crate::terms::Term::Abs;

pub fn apply(a: Term, b: Term) -> Term {
    App(Box::new(a), Box::new(b))
}

pub fn abstraction(c: char, b: Term) -> Term {
    Abs(c, Box::new(b))
}

    
