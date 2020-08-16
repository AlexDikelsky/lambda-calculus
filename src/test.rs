#![allow(unused_imports)]
use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::combinators::tru;
use crate::combinators::fls;
use crate::constants::var_a;
use crate::constants::var_b;

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

#[test]
fn test_substitutions() {
    let ident_a = Abs('a', Box::new(Var('a')));
    let ident_b = Abs('b', Box::new(Var('b')));

    let free_var = Var('f');
    let x = App(Box::new(ident_a.clone()), Box::new(Var('e'))).substitue(
            Substitution {
                to_replace: 'e',
                replace_with: Box::new(Var('n')),
            });
    let y = App(Box::new(ident_a.clone()), Box::new(Var('n')));
    //println!("{}, {}", x, y);
    assert!(x == y);


    // (λa.a)b
    let simple_sub = ident_a.clone().apply_abs(Var('b'));
    assert!(simple_sub == Var('b'));

    // (λa.a)(λb.b)
    let ident_to_ident = ident_a.clone().apply_abs(ident_b.clone());
    assert!(ident_to_ident == ident_b.clone());

    // (λa.a)(λa.a)
    let same_ident = ident_a.clone().apply_abs(ident_a.clone());
    assert!(same_ident == ident_a.clone());

    // (λa.b)a
    let free_trivial = Abs('a', Box::new(free_var.clone())).apply_abs(Var('a'));
    assert!(free_trivial == free_var.clone());

    // (λa.λb.a)(a)(b)
    let tru_test = tru().apply_abs(*var_a()).apply_abs(*var_b());
    assert!(tru_test == *var_a());

    // (λa.λb.a)(b)(a)
    let tru_test_2 = tru().apply_abs(*var_b()).apply_abs(*var_a());
    assert!(tru_test_2 == *var_b());

    
    //assert!(
    //    ident_a.clone().substitue(
    //        Substitution {
    //            to_replace: 'a',
    //            replace_with: Box::new(Var('b')),
    //        }) == ident_b.clone());

}

