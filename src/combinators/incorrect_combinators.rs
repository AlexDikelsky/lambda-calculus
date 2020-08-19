use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::terms::Term;

use crate::combinators::bool_combinators::{and, or, not};

use crate::aux::apply;
use crate::aux::abstraction;


// Incorrect xor operator
// λa.λb. or (and a b) (not (and a b))
pub fn wrong_xor() -> Term {
    let aandb = apply(apply(and(), Var('a')), Var('b'));
    
    abstraction('a',
        abstraction('b',
            apply(apply(or(), 
                        aandb.clone()), 
                  apply(not(), aandb))))

}

