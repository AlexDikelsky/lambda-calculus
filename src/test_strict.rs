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
use crate::constants::var_u;
use crate::constants::var_v;
use crate::constants::var_w;
use crate::constants::var_x;
use crate::constants::var_y;
use crate::constants::var_z;
use crate::constants::var_alpha;


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
fn vars_only() {
    let x = var_x().to_normal_form();
    assert!(x.clone().to_normal_form() == x);

    let xy = App(var_x(), var_y());
    assert!(
        xy.clone().to_normal_form() == xy);

    let x_yz = 
        App(var_x(),
            Box::new(App(var_y(), var_z())));
    assert!(
        x_yz.clone().to_normal_form() == x_yz);

}

#[test]
fn simple_normal_fms() {
    let t = *tru();
    dbg!(&t);
    assert!(t.clone().to_normal_form() == t);
    assert!(fls().to_normal_form() == *fls());
    assert!(id().to_normal_form() == *id());
    assert!(and().to_normal_form() == *and());
}

fn apply(a: Term, b: Term) -> Box<Term> {
    Box::new(App(Box::new(a), Box::new(b)))
}

fn abstraction(c: char, b: Term) -> Box<Term> {
    Box::new(Abs(c, Box::new(b)))
}

#[test]
fn a() {
    // (λx.xy)(λu.vuu)
    // reduces to vyy
    println!("Input: (λx.xy)(λu.vuu)");
    println!("Expected out: ((vy)y)");
    let x = 
      *apply(
        *abstraction(
            'x', *apply(*var_x(), *var_y())),
        *abstraction(
            'u', *apply(*apply(*var_v(), *var_u()), *var_u())),
    );
    let y = x.to_normal_form();
    let real = *apply(*apply(*var_v(), *var_y()), *var_y());
    assert!(real == y);
}

#[test]
fn b() {
    // (λx.λy.yx)uv
    // reduces to vu
    dbg!("(λx.λy.yx)uv");
    let x = 
        *apply(
            *apply(
                *abstraction(
                    'x',
                    *abstraction(
                        'y',
                        *apply(*var_y(), *var_x()))),
                    *var_u()),
                *var_v());
    let y = x.to_normal_form();
    let real = *apply(*var_v(), *var_u());
    assert!(real == y);
}

#[test]
fn c() {
    println!("(λx.x(x(yz))x)(λu.uv)");
    println!("yzvv(λu.uv)");

    let x =
        *apply(
            *abstraction(
                'x',
                *apply(
                    *apply(
                        *var_x(),
                        *apply(
                            *var_x(),
                            *apply(
                                *var_y(),
                                *var_z(),
                            )
                        )
                    ),
                    *var_x()
                )
            ),
            *abstraction(
                'u',
                *apply(
                    *var_u(),
                    *var_v(),
                )
            )
        );
    let real = 
        *apply(
            *apply(
                *apply(
                    *apply(
                        *var_y(),
                        *var_z(),
                    ),
                    *var_v(),
                ),
                *var_v(),
            ),
            *abstraction(
                'u',
                *apply(
                    *var_u(),
                    *var_v(),
                )
            )
        );
    dbg!(&x);
    dbg!(&real);
    let y = x.to_normal_form();
    assert!(y == real);
}



#[test]
fn d() {
    println!("(λx.xxy)(λy.yz)");
    println!("zzy");
    let x = 
        *apply(
            *abstraction(
                'x',
                *apply(
                    *apply(*var_x(), *var_x()),
                    *var_y()
                )
            ),
            *abstraction(
                'y',
                *apply(*var_y(), *var_z())
            ),
        );

    let y = x.to_normal_form();
    let real = *apply(
        *apply(*var_z(), *var_z()),
        *var_y());

    assert!(y == real);
}

#[test]
fn e() {
    println!("(λx.λy.xyy)(λu.uyx)");
    println!("(λα.αyxα)");
    let x =
        *apply(
            *abstraction(
                'x',
                *abstraction(
                    'y',
                    *apply(
                        *apply(
                            *var_x(),
                            *var_y()),
                        *var_y()),
            )),
            *abstraction(
                'u',
                *apply(
                    *apply(
                        *var_u(), *var_y()),
                    *var_x())));
    dbg!(&x);
    let y = x.to_normal_form();
    let real = 
        *abstraction(
            'α',
            *apply(
                *apply(
                    *apply(
                        *var_alpha(),
                        *var_y()),
                    *var_x()),
                *var_alpha()));

    dbg!(&real);
    assert!(real == y);
}





#[test]
fn test_and() {
    let flsfls = apply(*apply(*and(), *fls()), *fls());
    let norm = flsfls.to_normal_form();
    dbg!(&norm);
    assert!(norm == *fls());
}
