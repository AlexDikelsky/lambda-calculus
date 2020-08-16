#![allow(unused_imports)]
mod test;
mod terms;
mod subsitutions;
mod combinators;
mod constants;

use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;

fn main() {
    let id = Abs('x', Box::new(Var('x')));
    let tru = Abs('y', Box::new(Abs('z', Box::new(Var('y')))));
    let fls = Abs('a', Box::new(Abs('b', Box::new(Var('b')))));

    let and = Abs('c', Box::new(
                Abs('d', Box::new(
                  App(Box::new(App(Box::new(Var('c')), Box::new(Var('d')))), Box::new(fls.clone()))))));

    //let a = and.apply_abs(tru.clone()).apply_abs(tru.clone());
    dbg!(id.clone().apply_abs(id.clone()));
}



