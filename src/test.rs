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

#[test]
fn test_simple_subs() {
    let ident_a = Abs('a', Box::new(Var('a')));
    let ident_b = Abs('b', Box::new(Var('b')));

    let free_var = Var('f');


    // (λa.a)b
    let simple_sub = ident_a.clone().apply_abs(*var_b());
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

}

#[test]
fn test_simple_name_colision() {
    
    // (λx.λx.b)(b)(b)
    //     -> b
    let not_bad = 
        Abs('x',
            Box::new(Abs(
                    'x', var_b())))
        .apply_abs(*var_b())
        .apply_abs(*var_b());
    
    assert!(not_bad == *var_b());


    // (λx.λx.b)(a)(a)
    //     -> b
    let bad_colision = 
        Abs('x',
            Box::new(Abs(
                    'x', var_b())))
        .apply_abs(*var_a())
        .apply_abs(*var_a());
    assert!(bad_colision.clone() == *var_b());

    // (λx.(λy.λz. x y)y)
    //      y is free and bound in different places
    //
    //    dbg -> (λx.((λy.(λz.(x y))) y))
    let y_bound_free = 
        Abs('x', 
            Box::new(App(Box::new(Abs('y', Box::new(Abs('z', Box::new(App(var_x(), var_y())))))), var_y())));

    // (λx.(λy.λz. x y)y)(a)(b)
    // ((λy.λz. a y)y)(b)
    //   -> is a normal form
    let lhs_y_free_1 = y_bound_free.clone().apply_abs(*var_a()).apply_abs(*var_b());

    // ((λy.λz. a y)y)(b)
    let rhs_y_free_1 = App(
            Box::new(App(Box::new(Abs('y', Box::new(Abs('z', Box::new(App(var_a(), var_y())))))), var_y())), 
            var_b());

    assert!(lhs_y_free_1 == rhs_y_free_1);

}

#[test]
fn mistaken_test_capture() {
    // λx.xy
    let failed_capture_init_x = 
        Abs('x', Box::new(App(var_x(), var_y())));

    // λy.xy
    let failed_capture_init_y = 
        Abs('y', Box::new(App(var_x(), var_y())));

    // (λx.xy)(λy.xy)
    //   -> 
    //      ((λy.xy)y)
    let failed_capture_x = 
        failed_capture_init_x.apply_abs(failed_capture_init_y);

    let failed_test = App(
                Box::new(Abs('y', Box::new(App(var_x(), var_y())))),
                var_y());

    assert!(&failed_test == &failed_capture_x);

}

#[test]
fn substitution_test() {

    // λy.x
    let apply_to = Abs('y', var_x()) ;
    
    // λz.zw
    let arg = Abs('z', Box::new(App(var_z(), var_w())));

    let complete = apply_to.substitue(
        Substitution {
            to_replace: 'x',
            replace_with: arg,
        });

    let rhs = Abs('y', Box::new(Abs('z', 
                                    Box::new(App(var_z(), 
                                                 var_w()))
                                    )));

    dbg!(&complete, &rhs);

    assert!(complete == rhs);
}

#[test]
fn capture_book() {

    // λz.x
    let apply_to = Abs('z', var_x());

    let applied = apply_to.substitue(
        Substitution {
            to_replace: 'x',
            replace_with: *var_z(),
        });

    // make sure it's not the identity function, because
    // that would mean that the previous function changed
    // λz.y to λz.z
    let rhs = Abs('z', var_z());

    assert!(&applied != &rhs);
}

fn apply(a: Term, b: Term) -> Box<Term> {
    Box::new(App(Box::new(a), Box::new(b)))
}

#[test]
fn test_and() {
    //dbg!(and().strict_apply(*fls()).strict_apply(*fls()));
    let flsfls = apply(*apply(*and(), *fls()), *fls());
    dbg!(&flsfls);
    assert!(flsfls == fls());
}


