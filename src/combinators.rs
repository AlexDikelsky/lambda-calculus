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

// λx.λy.x
pub fn tru() -> Term {
    abstraction('y', abstraction( 'z', Var('y')))
}

// λx.λy.y
pub fn fls() -> Term {
    abstraction('a', (abstraction('b', (Var('b')))))
}

// λb.λc. b c fls
pub fn and() -> Term {
    abstraction('b', 
        abstraction('c', 
            apply(
                apply(Var('b'), Var('c')), 
                fls())
            )
        )

}

// Switch first and second arg
//   K in combinatory logic
#[allow(non_snake_case)]
pub fn K() -> Term {
    abstraction('x',
                abstraction('y',
                            apply(Var('y'), Var('x'))))
}

// Ω = (λx. x x)(λx. x x)
pub fn omega_parts() -> Term {
    abstraction('x', apply( Var('x'), Var('x'),))
}
