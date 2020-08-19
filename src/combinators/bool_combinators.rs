use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::terms::Term;

use crate::aux::apply;
use crate::aux::abstraction;

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

// λb.λc. a tru fls
pub fn or() -> Term {
    abstraction('a',
        abstraction('b',
            apply(
                apply(
                    Var('a'), tru()),
                Var('b'))))
}

// λx. fls tru
pub fn not() -> Term {
    abstraction('x',
        apply(apply(Var('x'), fls()), tru()))
}

pub fn xor() -> Term {
    let a_and_b = apply(apply(and(), Var('a')), Var('b'));
    let not_a_and_not_b = 
        apply(apply(and(), apply(not(), Var('a'))), apply(not(), Var('b')));

    apply(apply(or(), a_and_b), not_a_and_not_b)
}

