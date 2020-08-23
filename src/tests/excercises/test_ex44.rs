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
use crate::terms::Term::Var;
use crate::terms::Term;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::combinators::one_arg_combinators::id;
use crate::combinators::bool_combinators::tru;
use crate::combinators::bool_combinators::fls;
use crate::combinators::bool_combinators::and;
use crate::combinators::two_arg_combinators::swap;

use crate::aux::apply;
use crate::aux::abstraction;

#[test]
fn ei() {
    let a =
        abstraction(
            'x', abstraction('y', apply(apply(Var('x'), Var('y')), Var('y'))));
    let b = apply(a, Var('u'));
    let c = apply(b, Var('v'));

    let real = apply(apply(Var('u'), Var('v')), Var('v'));

    assert!(c.to_normal_form() == real);
}

#[test]
fn eii() {
    let a = swap();
    let b = apply(a, apply(Var('u'), Var('v')));

    let c = apply(apply(b, Var('z')), Var('w'));

    let real = 
        apply(apply(Var('z'), apply(Var('u'), Var('v'))), Var('w'));

    assert!(c.to_normal_form() == real);
}

#[test]
fn eiii() {
    let a = apply(tru(), id());

    let real = fls();

    assert!(a.to_normal_form() == real);
}

#[test]
fn eiv() {
    let inner = apply(apply(Var('x'), Var('z')), apply(Var('y'), Var('z')));
    let a = 
        abstraction('x', abstraction('y', abstraction('z', inner)));

    let b = apply(a, tru()).to_normal_form();

    println!("{}", &b);
    println!("{}", fls());
    assert!(b == fls());
}

