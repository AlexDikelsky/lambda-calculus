use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::terms::Term;

use crate::aux::apply;
use crate::aux::abstraction;

// λx.x
pub fn id() -> Term {
    Abs('x', Box::new(Var('x')))
}
