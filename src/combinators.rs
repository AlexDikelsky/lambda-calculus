use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::terms::Term;
use crate::constants::var_b;
use crate::constants::var_c;
use crate::constants::var_x;

// λx.x
pub fn id() -> Box<Term> {
    Box::new(Abs('x', Box::new(Var('x'))))
}

// λx.λy.x
pub fn tru() -> Box<Term> {
    Box::new(Abs('y', Box::new(Abs('z', Box::new(Var('y'))))))
}

// λx.λy.y
pub fn fls() -> Box<Term> {
    Box::new(Abs('a', Box::new(Abs('b', Box::new(Var('b'))))))
}

// λb.λc. b c fls
pub fn and() -> Box<Term> {
    Box::new(
        Abs('b', Box::new(
            Abs('c', Box::new(
                App(Box::new(
                    App(var_b(), var_c())), 
                    fls()))
                )
            ))
        )

}

// Ω = (λx. x x)(λx. x x)
pub fn omega() -> Box<Term> {
    let o = Box::new(
        Abs('x', Box::new(
            App(var_x(), var_x()))));

    Box::new(
        App(
            Box::new(o.clone()), 
            Box::new(*o)))
}
