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

use crate::aux::{ apply, abstraction, parse };
use crate::terms::Term;
use crate::terms::Term::{ Var };
use crate::combinators::bool_combinators::{ not, tru, fls };

pub fn c0() -> Term {
    parse("(λs.(λn.n))").unwrap()
}

// Generate next church numeral
//    Applies the first argument to the second again
pub fn succ() -> Term {
    parse("(λx.(λs.(λn.(s((xs)n)))))").unwrap()
}

// Another successor
//   Compose same number of times, but change the thing composed with
pub fn succ2() -> Term {
    //let c1 = "λs.(λn.(sn))";
    //let id_str = "λx.x";
    //parse(&format!("(λx.(λy.(λz.((xy)({})))))", c1)).unwrap()
    //parse(&format!("(λx.((x(λy.y)({}))))", c1)).unwrap()
    //
    //  Fails because making the function to compose the identity
    //  function removes the fact that it was called
    //parse("(λx.((x(λy.y))(λs.(λn.(sn)))))").unwrap()

    //  Fails because composes y with the function that returns 1,
    //  rather than the function that returns 0
    //  To get this to work, need to apply another y rather than this
    //parse("(λx.(λy.((xy)(λn.(yn)))))").unwrap()

    //parse("(λx.(λy.(λz.((xy)((λs.(λn.(sn)))y)))))").unwrap()
    //parse("(λx.(λy.((xy)((λs.(λn.(sn)))y))))").unwrap()
    //parse("(λx.(λy.((x(λn.(yn)))(λn.(yn)))))").unwrap()
    //
    //This works by changing the base rather than composing a new function
    parse("(λx.(λy.(λz.((xy)(yz)))))").unwrap()
}

// Since a number is the first function composed to the second
// n times, you can check the parity of the number by making
// the first function negation and the second function true
pub fn is_even() -> Term {
    abstraction(
        'x',
        apply(apply(Var('x'), not()), tru()))
}

// Similar here, but start with fls rather than tru.
pub fn is_odd() -> Term {
    abstraction(
        'x',
        apply(apply(Var('x'), not()), fls()))
}
