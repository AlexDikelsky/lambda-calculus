use crate::aux::{ apply, abstraction, parse };
use crate::terms::Term;

pub fn c0() -> Term {
    parse("(λs.(λn.n))").unwrap()
}

// Generate next church numeral
pub fn succ() -> Term {
    parse("(λx.(λs.(λn.(s((xs)n)))))").unwrap()
}

pub fn plus() -> Term {
    parse("(λm.(λn.(λs.(λz.(((m s) ((n s) z)))))))").unwrap()
}

pub fn iszro() -> Term {
    let f = "(λx.(λy.y))";
    let t = "(λx.(λy.x))";
    let s = format!("(λm.((m{}) {})", f, t);

    parse(&s).unwrap()
}

//// Generate nth church numeral
//pub fn cn(n: usize) -> Term {
//    match cn <= 0 {
//        true  => c0(),
//        false => succ(cn(n-1)),
//    }
//}
