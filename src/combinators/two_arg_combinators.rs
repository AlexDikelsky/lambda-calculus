use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::terms::Term;

use crate::aux::apply;
use crate::aux::abstraction;

// Switch first and second arg
pub fn swap() -> Term {
    abstraction('x',
                abstraction('y',
                            apply(Var('y'), Var('x'))))
}

