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
pub fn succ() -> Term {
    parse("(λx.(λs.(λn.(s((xs)n)))))").unwrap()
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
