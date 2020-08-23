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
