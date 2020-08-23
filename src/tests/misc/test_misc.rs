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
use crate::combinators::bool_combinators::tru;
use crate::combinators::bool_combinators::fls;
use crate::combinators::bool_combinators::and;

#[test]
fn test_free_vars() {
    let x = Abs('a', Box::new(Var('a')));
    let y = Abs('b', Box::new(Var('b')));
    let f = App(Box::new(x.clone()), Box::new(y.clone()));
    //assert!(f.clone().free_vars() == vec![]);

    let z = Abs('c', Box::new(App(Box::new(x.clone()), Box::new(Var('e')))));
    assert!(z.clone().free_vars() == vec!['e']);

    let a = App(Box::new(Var('a')), Box::new(Var('b')));
    assert!(a.free_vars() == vec!['a', 'b']);


}
