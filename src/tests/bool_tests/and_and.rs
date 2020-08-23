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

use crate::aux::apply;
use crate::aux::abstraction;

//fn fls_and_fls() -> Term {
//    apply(apply(and(), fls()), fls())
//}
//fn fls_and_tru() -> Term {
//    apply(apply(and(), fls()), tru())
//}
//fn tru_and_fls() -> Term {
//    apply(apply(and(), tru()), fls())
//}
//fn tru_and_tru() -> Term {
//    apply(apply(and(), tru()), tru())
//}
//
//fn ands(n: usize, op: Term) -> Term {
//    (0..n).fold(op.clone(), |a, b| apply(a, op.clone()))
//}
//
//#[test]
//fn ntrutru() {
//    //assert!(ands(4, tru_and_tru()).to_normal_form() == ands(10, tru_and_tru()).to_normal_form());
//    //assert!(apply(tru_and_tru(), tru_and_tru()).to_normal_form() == tru_and_tru().to_normal_form());
//
//
//    let s = apply(apply(and(), tru_and_tru()), tru_and_tru()).to_normal_form();
//    dbg!(&s);
//    
//    assert!(s == tru());
//}
//
//#[test]
//fn ntrufls() {
//    assert!(ands(4, tru_and_fls()).to_normal_form() == ands(10, tru_and_fls()).to_normal_form());
//    assert!(ands(5, tru_and_fls()).to_normal_form() == tru_and_fls().to_normal_form());
//}
//#[test]
//fn nflstru() {
//    assert!(ands(4, fls_and_tru()).to_normal_form() == ands(10, fls_and_tru()).to_normal_form());
//    assert!(ands(5, fls_and_tru()).to_normal_form() == fls_and_tru().to_normal_form());
//}
//#[test]
//fn nflsfls() {
//    assert!(ands(4, fls_and_fls()).to_normal_form() == ands(10, fls_and_fls()).to_normal_form());
//    assert!(ands(5, fls_and_fls()).to_normal_form() == fls_and_fls().to_normal_form());
//}
