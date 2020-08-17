#![allow(unused_imports)]
use crate::terms::Term::Var;
use crate::terms::Term;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::combinators::tru;
use crate::combinators::fls;
use crate::combinators::and;
use crate::constants::var_a;
use crate::constants::var_b;
use crate::constants::var_w;
use crate::constants::var_x;
use crate::constants::var_y;
use crate::constants::var_z;

#[test]
fn test_free_vars() {
    let x = Abs('a', Box::new(Var('a')));
    let y = Abs('b', Box::new(Var('b')));
    let f = App(Box::new(x.clone()), Box::new(y.clone()));
    assert!(f.clone().free_vars() == vec![]);

    let z = Abs('c', Box::new(App(Box::new(x.clone()), Box::new(Var('e')))));
    assert!(z.clone().free_vars() == vec!['e']);

    let a = App(Box::new(Var('a')), Box::new(Var('b')));
    assert!(a.free_vars() == vec!['a', 'b']);


}
