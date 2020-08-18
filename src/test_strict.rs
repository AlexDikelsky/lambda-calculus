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

use crate::aux::apply;
use crate::aux::abstraction;


#[test]
fn test_simple_subs() {
    let ident_a = Abs('a', Box::new(Var('a')));
    let ident_b = Abs('b', Box::new(Var('b')));

    let free_var = Var('f');


    // (λa.a)b
    let simple_sub = apply(ident_a.clone(),
                                  Var('b'));
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
fn vars_only() {
    let x = Var('x').to_normal_form();
    assert!(x.clone().to_normal_form() == x);

    let xy = apply(Var('x'), Var('y'));
    assert!(
        xy.clone().to_normal_form() == xy);

    let x_yz = 
        apply(Var('x'),
            apply(Var('y'), Var('z')));
    assert!(
        x_yz.clone().to_normal_form() == x_yz);

}

#[test]
fn simple_normal_fms() {
    let t = tru();
    dbg!(&t);
    assert!(t.clone().to_normal_form() == t);
    assert!(fls().to_normal_form() == fls());
    assert!(id().to_normal_form() == id());
    assert!(and().to_normal_form() == and());
}





#[test]
fn rename_1() {
    println!("(λz.x)z");
    println!("x");
    let x =
        apply(
            abstraction(
                'z',
                Var('x')),
            Var('z'));

    assert!(x.to_normal_form() == Var('x'));
}

#[test]
fn rename_2() {
    println!("(λz.xz)z");
    println!("xα");
    let x =
        apply(
            abstraction(
                'z',
                apply(
                    Var('x'),
                    Var('z'),
                ),
            ),
            Var('z'));
    assert!(x.to_normal_form() == apply(Var('x'), Var('z')));
}

#[test]
fn rename_3() {
    println!("(λz.xz)(λa.z)");
    println!("x(λa.z)");

    let x =
        apply(
            abstraction(
                'z',
                apply(
                    Var('x'),
                    Var('z'),
                ),
            ),
            abstraction(
                'a',
                Var('z')));

    let real = 
        apply(
            Var('x'),
            abstraction('a', Var('z')));

    assert!(x.to_normal_form() == real);
}




