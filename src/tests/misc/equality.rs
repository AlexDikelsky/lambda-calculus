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
