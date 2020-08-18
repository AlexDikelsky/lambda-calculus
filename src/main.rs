#![allow(unused_imports)]
mod test_lazy;
mod test_strict;
mod test_misc;
mod test_normal;
mod terms;
mod subsitutions;
mod combinators;
mod constants;
mod letter_list;

use crate::terms::Term;
use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::combinators::tru;
use crate::combinators::and;
use crate::combinators::fls;
use crate::constants::var_a;
use crate::constants::var_b;
use crate::constants::var_u;
use crate::constants::var_v;
use crate::constants::var_w;
use crate::constants::var_x;
use crate::constants::var_y;
use crate::constants::var_z;
use crate::combinators::omega_parts;

fn apply(a: Term, b: Term) -> Box<Term> {
    Box::new(App(Box::new(a), Box::new(b)))
}

fn abstraction(c: char, b: Term) -> Box<Term> {
    Box::new(Abs(c, Box::new(b)))
}

fn main() {
    println!("Input: (λx.xy)(λu.vuu)");
    println!("Expected out: ((vy)y)");
    let x = 
      *apply(
        *abstraction(
            'x', *apply(*var_x(), *var_y())),
        *abstraction(
            'u', *apply(*apply(*var_v(), *var_u()), *var_u())),
    );
    let y = x.to_normal_form();
    let real = *apply(*apply(*var_v(), *var_y()), *var_y());

    //let id = Abs('x', Box::new(Var('x')));
    //let tru = Abs('y', Box::new(Abs('z', Box::new(Var('y')))));
    //let fls = Abs('a', Box::new(Abs('b', Box::new(Var('b')))));

    //let and = Abs('c', Box::new(
    //            Abs('d', Box::new(
    //              App(Box::new(App(Box::new(Var('c')), Box::new(Var('d')))), Box::new(fls.clone()))))));

    ////let a = and.apply_abs(tru.clone()).apply_abs(tru.clone());
    //let y_bound_free = 
    //    Abs('x', 
    //        Box::new(App(Box::new(Abs('y', Box::new(Abs('z', Box::new(App(var_x(), var_y())))))), var_y())));

    //// (λx.(λy.λz. x y)y)(a)(b)
    //// ((λy.λz. a y)y)(b)
    ////   -> is a normal form
    //let lhs_y_free_1 = y_bound_free.clone().apply_abs(*var_a()).apply_abs(*var_b());

    //// ((λy.λz. a y)y)(b)
    //let rhs_y_free_1 = App(
    //        Box::new(App(Box::new(Abs('y', Box::new(Abs('z', Box::new(App(var_a(), var_y())))))), var_y())), 
    //        var_b());


    //dbg!(&lhs_y_free_1);
    //dbg!(&rhs_y_free_1);
    
    //dbg!(and().apply_abs(*fls()).apply_abs(*fls()));
}



