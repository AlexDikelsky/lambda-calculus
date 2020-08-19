#![allow(unused_imports)]
use crate::terms::Term::Var;
use crate::terms::Term;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::combinators::one_arg_combinators::id;
use crate::combinators::bool_combinators::tru;
use crate::combinators::bool_combinators::fls;
use crate::combinators::bool_combinators::and;
use crate::aux::apply;
use crate::aux::abstraction;


#[test]
fn test_simple_subs() {
    let ident_a = Abs('a', Box::new(Var('a')));
    let ident_b = Abs('b', Box::new(Var('b')));

    let free_var = Var('f');


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
    let tru_test = tru().apply_abs(Var('a')).apply_abs(Var('b'));
    assert!(tru_test == Var('a'));

    // (λa.λb.a)(b)(a)
    let tru_test_2 = tru().apply_abs(Var('b')).apply_abs(Var('a'));
    assert!(tru_test_2 == Var('b'));

}

#[test]
fn both_false() {
    // (λa.(λx.x)c))((λa.(λx.x)c)
    // c
    let part = abstraction('a', apply(id(), Var('c')));
    let x = apply(part.clone(), part).to_normal_form();

    assert!(x == Var('c'));
}


#[test]
fn test_simple_name_colision() {
    
    // (λx.λx.b)(b)(b)
    //     -> b
    let not_bad = 
        abstraction('x',
            abstraction(
                    'x', Var('b')))
        .apply_abs(Var('b'))
        .apply_abs(Var('b'));
    
    assert!(not_bad == Var('b'));


    // (λx.λx.b)(a)(a)
    //     -> b
    let bad_colision = 
        abstraction('x',
            abstraction(
                    'x', Var('b')))
        .apply_abs(Var('a'))
        .apply_abs(Var('a'));
    assert!(bad_colision.clone() == Var('b'));

    // (λx.(λy.λz. x y)y)
    //      y is free and bound in different places
    //
    //    dbg -> (λx.((λy.(λz.(x y))) y))
    let y_bound_free = 
        abstraction('x', 
            apply(abstraction('y', 
                              abstraction('z', apply(Var('x'), Var('y')))), 
                  Var('y')));

    // (λx.(λy.λz. x y)y)(a)(b)
    // ((λy.λz. a y)y)(b)
    //   -> is a normal form
    let lhs_y_free_1 = y_bound_free.clone().apply_abs(Var('a')).apply_abs(Var('b'));

    // ((λy.λz. a y)y)(b)
    let rhs_y_free_1 = apply(
            apply(abstraction('y', 
                              abstraction('z', 
                                          apply(
                                              Var('a'), 
                                              Var('y')))),
                  Var('y')), 
            Var('b'));

    assert!(lhs_y_free_1 == rhs_y_free_1);

}

#[test]
fn mistaken_test_capture() {
    // λx.xy
    let failed_capture_init_x = 
        abstraction('x', apply(Var('x'), Var('y')));

    // λy.xy
    let failed_capture_init_y = 
        abstraction('y', apply(Var('x'), Var('y')));

    // (λx.xy)(λy.xy)
    //   -> 
    //      ((λy.xy)y)
    let failed_capture_x = 
        failed_capture_init_x.apply_abs(failed_capture_init_y);

    let failed_test = apply(
                abstraction('y', apply(Var('x'), Var('y'))),
                Var('y'));

    assert!(&failed_test == &failed_capture_x);

}

#[test]
fn substitution_test() {

    // λy.x
    let apply_to = abstraction('y', Var('x')) ;
    
    // λz.zw
    let arg = abstraction('z', apply(Var('z'), Var('w')));

    let complete = apply_to.substitue(
        Substitution {
            to_replace: 'x',
            replace_with: arg,
            debug: true
        });

    let rhs = abstraction('y', abstraction('z', 
                                    apply(Var('z'), 
                                                 Var('w'))
                                    ));

    dbg!(&complete, &rhs);

    assert!(complete == rhs);
}

#[test]
fn capture_book() {

    // λz.x
    let apply_to = abstraction('z', Var('x'));

    let applied = apply_to.substitue(
        Substitution {
            to_replace: 'x',
            replace_with: Var('z'),
            debug: true
        });

    // make sure it's not the identity function, because
    // that would mean that the previous function changed
    // λz.y to λz.z
    let rhs = abstraction('z', Var('z'));

    assert!(&applied != &rhs);
}


//#[test]
//fn test_and() {
//    //dbg!(and().strict_apply(*fls()).strict_apply(*fls()));
//    let flsfls = apply(*apply(*and(), *fls()), *fls());
//    let norm = flsfls.to_normal_form();
//    dbg!(&norm);
//    assert!(norm == *fls());
//}
//
//
