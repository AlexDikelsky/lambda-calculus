#![allow(unused_imports)]
use crate::terms::Term::Var;
use crate::terms::Term;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::combinators::id;
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
fn test_simple_subs() {
    let ident_a = Abs('a', Box::new(Var('a')));
    let ident_b = Abs('b', Box::new(Var('b')));

    let free_var = Var('f');


    // (λa.a)b
    let simple_sub = App(Box::new(ident_a.clone()),
                                  var_b());
    //dbg!(&simple_sub);
    assert!(simple_sub.to_normal_form() == Var('b'));

    // (λa.a)(λb.b)
    let ident_to_ident = 
        App(Box::new(ident_a.clone()), 
            Box::new(ident_b.clone()));
    dbg!(&ident_to_ident);
    assert!(ident_to_ident.to_normal_form() == ident_b.clone());

    // (λa.a)(λa.a)
    let same_ident = 
        App(Box::new(ident_a.clone()),
            Box::new(ident_a.clone()));
    assert!(same_ident.to_normal_form() == ident_a.clone());

    // (λa.b)a
    let respect_scope = 
        App(Box::new(Abs('a', Box::new(free_var.clone()))),
            Box::new(Var('a')));
    assert!(respect_scope.to_normal_form() == free_var.clone());

}

#[test]
fn normal_fms() {
    assert!(tru().to_normal_form() == *tru());
    assert!(fls().to_normal_form() == *fls());
    assert!(id().to_normal_form() == *id());
    assert!(and().to_normal_form() == *and());
}
