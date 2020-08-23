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
use crate::aux::apply;
use crate::aux::abstraction;
use crate::terms::Term::Var;
use crate::terms::Term;
use crate::terms::Term::Abs;
use crate::terms::Term::App;


#[test]
fn fv_eq_fv() {
    println!("a == a");
    assert!(Var('a') == Var('a'));

    println!("a == b");
    assert!(Var('a') == Var('b'));

    println!("b == a");
    assert!(Var('b') == Var('a'));
}
