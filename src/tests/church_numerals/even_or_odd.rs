use crate::aux::{ apply, abstraction };
use crate::combinators::church_numerals::{ c0, succ, is_even, is_odd };
use crate::combinators::bool_combinators::{ tru, fls, not };

// for all n,
// is_even ((succ succ)^n c0) = tru 
#[test]
fn evens_are_even() {
    let mut n = c0();
    for i in 0..10 {
        dbg!(i*2, &n);
        let v = apply(is_even(), n.clone()).to_normal_form();
        assert!(v == tru());
        n = apply(succ(), apply(succ(), n));
    }
}

#[test]
fn evens_arent_odd() {
    let mut n = apply(succ(), c0()).to_normal_form();

    for i in 0..10 {
        dbg!(i*2 + 1, &n);
        let v = apply(is_even(), n.clone()).to_normal_form();
        assert!(v == fls());
        n = apply(succ(), apply(succ(), n));
    }
}

#[test]
fn odds_are_odd() {
    let mut n = apply(succ(), c0()).to_normal_form();

    for i in 0..10 {
        dbg!(i*2 + 1, &n);
        let v = apply(is_odd(), n.clone()).to_normal_form();
        assert!(v == tru());
        n = apply(succ(), apply(succ(), n));
    }
}

#[test]
fn odds_arent_even() {
    let mut n = c0();

    for i in 0..10 {
        dbg!(i*2, &n);
        let v = apply(is_odd(), n.clone()).to_normal_form();
        assert!(v == fls());
        n = apply(succ(), apply(succ(), n));
    }
}
