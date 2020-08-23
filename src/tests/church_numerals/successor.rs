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
