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

#[test]
fn fls_and_fls() {
    let flsfls = apply(apply(and(), fls()), fls());
    let norm = flsfls.to_normal_form();
    dbg!(&norm);
    assert!(norm == fls());
}
#[test]
fn fls_and_tru() {
    let flstru = apply(apply(and(), fls()), tru());
    let norm = flstru.to_normal_form();
    dbg!(&norm);
    assert!(norm == fls());
}
#[test]
fn tru_and_fls() {
    let trufls = apply(apply(and(), tru()), fls());
    let norm = trufls.to_normal_form();
    dbg!(&norm);
    assert!(norm == fls());
}
#[test]
fn tru_and_tru() {
    let trutru = apply(apply(and(), tru()), tru());
    let norm = trutru.to_normal_form();
    dbg!(&norm);
    assert!(norm == tru());
}
