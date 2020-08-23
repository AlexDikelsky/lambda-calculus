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
use crate::aux::{ apply, abstraction, parse};
use crate::combinators::church_numerals::{ c0, succ };

#[test]
fn succX_is_succX() {
    // For all x, s(x) = s(x)

    let mut n = c0();
    for _ in 0..10 {
        assert!(apply(succ(), n.clone()).to_normal_form() == apply(succ(), n.clone()).to_normal_form());
        n = apply(succ(), n.clone()).to_normal_form();
    }
}

#[test]
fn succ_zro_is_1() {

    let one = apply(succ(), c0()).to_normal_form();

    let real = parse("(λx.(λy.(xy)))").unwrap();

    assert!(real == one);
    assert!(!(real == c0()));

}

#[test]
fn succ_n_is_n_plus_1() {
    let mut n = c0();
    let mut n_plus_one = apply(succ(), c0()).to_normal_form();

    for _ in 0..10 {
        assert!(apply(succ(), n.clone()).to_normal_form() == n_plus_one);

        n = apply(succ(), n).to_normal_form();
        n_plus_one = apply(succ(), n_plus_one).to_normal_form();
    }
}

//#[test]
//fn n_less_than_succ_n() {
//    let mut n = c0();
//    let mut n_plus_one = apply(succ(), c0()).to_normal_form();
//
//    for _ in 0..10 {
//        assert!(n < n_plus_one);
//
//        n = apply(succ(), n).to_normal_form();
//        n_plus_one = apply(succ(), n_plus_one).to_normal_form();
//    }
//}
