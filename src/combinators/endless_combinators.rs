use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::terms::Term;

use crate::aux::apply;
use crate::aux::abstraction;

// Ω = (λx. x x)(λx. x x)
pub fn omega_parts() -> Term {
    abstraction('x', apply( Var('x'), Var('x'),))
}

pub fn omega() -> Term {
    apply(omega_parts(), omega_parts())
}
