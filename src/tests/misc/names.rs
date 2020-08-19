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
use crate::combinators::swap;

use crate::aux::apply;
use crate::aux::abstraction;


#[test]
fn all_greek() {
    let f = abstraction('a', abstraction('b', Var('b')));
    let g = abstraction('c', abstraction('d', Var('d')));

    assert!(f == g);
}


