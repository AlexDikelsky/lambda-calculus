//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.
//
//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY of FITNESS FOR A PARTICULAR PURPOSE. See the
//GNU General Public License for more details.
//
//You should have recieved a copy of the GNU General Public License
//along with this program. If not, see <https://www.gnu.org/licenses/>
#![allow(unused_imports)]
mod terms;
mod subsitutions;
mod combinators;
mod letter_list;
mod aux;
mod tests;

//use crate::terms::Term;
//use crate::terms::Term::Var;
//use crate::terms::Term::Abs;
//use crate::terms::Term::App;
//use crate::combinators::bool_combinators::tru;
//use crate::combinators::bool_combinators::and;
//use crate::combinators::bool_combinators::fls;
//use crate::combinators::omega_parts;
//use crate::combinators::one_arg_combinators::id;
//use crate::combinators::xor;
use crate::aux::apply;
use crate::aux::abstraction;
use crate::aux::parse;

use crate::combinators::church_numerals::*;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub lambda);

//fn main() {
//    dbg!(xor());
//    //let part = abstraction('a', apply(id(), Var('c')));
//    //let x = apply(part.clone(), part).to_normal_form();
//    //dbg!(x);
//}

fn main() {
    let b = apply(succ2(), c0()).to_normal_form();
    dbg!(&b);
    let b = apply(succ2(), b).to_normal_form();
    dbg!(&b);
    let b = apply(succ2(), b).to_normal_form();
    dbg!(&b);

    //let a_to_b = parse("(λa.(λb.(ab)))").unwrap();
    //let one = apply(succ(), c0()).to_normal_form();
    //let two = apply(succ(), one.clone()).to_normal_form();
    //let three = apply(succ(), two.clone()).to_normal_form();
    //let four = apply(succ(), three.clone()).to_normal_form();
    //let five = apply(succ(), four.clone()).to_normal_form();
    //let six = apply(succ(), five.clone()).to_normal_form();

    //let a = six.clone();
    //let b = six.clone();

    //println!("{}", apply(apply(a_to_b.clone(), a.clone()), b.clone()).to_normal_form());


}

//fn main() {
//    dbg!(lambda::TermParser::new().parse("u").is_ok());
//    dbg!(lambda::TermParser::new().parse("(λu.u)").is_ok());
//    dbg!(lambda::TermParser::new().parse("u"));
//}

//fn main() {
//
//    let omega = apply(omega_parts(), omega_parts());
//    dbg!(&omega);
//    omega.to_normal_form();
//
//
//    //println!("Input: (λx.xy)(λu.vuu)");
//    //println!("Expected out: ((vy)y)");
//    //let x = 
//    //  *apply(
//    //    *abstraction(
//    //        'x', *apply(*var_x(), *var_y())),
//    //    *abstraction(
//    //        'u', *apply(*apply(*var_v(), *var_u()), *var_u())),
//    //);
//    //let y = x.to_normal_form();
//    //let real = *apply(*apply(*var_v(), *var_y()), *var_y());
//
//    //let id = Abs('x', Box::new(Var('x')));
//    //let tru = Abs('y', Box::new(Abs('z', Box::new(Var('y')))));
//    //let fls = Abs('a', Box::new(Abs('b', Box::new(Var('b')))));
//
//    //let and = Abs('c', Box::new(
//    //            Abs('d', Box::new(
//    //              App(Box::new(App(Box::new(Var('c')), Box::new(Var('d')))), Box::new(fls.clone()))))));
//
//    ////let a = and.apply_abs(tru.clone()).apply_abs(tru.clone());
//    //let y_bound_free = 
//    //    Abs('x', 
//    //        Box::new(App(Box::new(Abs('y', Box::new(Abs('z', Box::new(App(var_x(), var_y())))))), var_y())));
//
//    //// (λx.(λy.λz. x y)y)(a)(b)
//    //// ((λy.λz. a y)y)(b)
//    ////   -> is a normal form
//    //let lhs_y_free_1 = y_bound_free.clone().apply_abs(*var_a()).apply_abs(*var_b());
//
//    //// ((λy.λz. a y)y)(b)
//    //let rhs_y_free_1 = App(
//    //        Box::new(App(Box::new(Abs('y', Box::new(Abs('z', Box::new(App(var_a(), var_y())))))), var_y())), 
//    //        var_b());
//
//
//    //dbg!(&lhs_y_free_1);
//    //dbg!(&rhs_y_free_1);
//    
//    //dbg!(and().apply_abs(*fls()).apply_abs(*fls()));
//}





