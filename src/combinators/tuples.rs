use crate::aux::*;
use crate::combinators::bool_combinators::{ tru, fls };

pub fn pair() -> Term {
    parse("(λf.(λs.(λb.((bf)s))))").unwrap()
}

pub fn fst() -> Term {
    abstraction('p', apply(Var('p'), tru()))
}

pub fn snd() -> Term {
    abstraction('p', apply(Var('p'), fls()))
}
