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

// λa.λb.a(¬b)b
pub fn xor() -> Term {
    abstraction('a', abstraction('b', apply(apply(Var('a'), apply(not(), Var('b'))), Var('b'))))
}

