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
use crate::combinators::swap;

use crate::aux::apply;
use crate::aux::abstraction;

// These are from "Lambda-Calculus and Combinators: an Introduction"
//   by Hindley and Seldin

#[test]
fn a() {
    // (λx.xy)(λu.vuu)
    // reduces to vyy
    println!("Input: (λx.xy)(λu.vuu)");
    println!("Expected out: ((vy)y)");
    let x = 
      apply(
        abstraction(
            'x', apply(Var('x'), Var('y'))),
        abstraction(
            'u', apply(apply(Var('v'), Var('u')), Var('u'))),
    );
    let y = x.to_normal_form();
    let real = apply(apply(Var('v'), Var('y')), Var('y'));
    assert!(real == y);
}

#[test]
fn b() {
    // (λx.λy.yx)uv
    // reduces to vu
    dbg!("(λx.λy.yx)uv");
    let x = 
        apply(apply(swap(), Var('u')), Var('v'));

    let y = x.to_normal_form();
    let real = apply(Var('v'), Var('u'));
    assert!(real == y);
}

#[test]
fn c() {
    println!("(λx.x(x(yz))x)(λu.uv)");
    println!("yzvv(λu.uv)");

    let x =
        apply(
            abstraction(
                'x',
                apply(
                    apply(
                        Var('x'),
                        apply(
                            Var('x'),
                            apply(
                                Var('y'),
                                Var('z'),
                            )
                        )
                    ),
                    Var('x')
                )
            ),
            abstraction(
                'u',
                apply(
                    Var('u'),
                    Var('v'),
                )
            )
        );
    let real = 
        apply(
            apply(
                apply(
                    apply(
                        Var('y'),
                        Var('z'),
                    ),
                    Var('v'),
                ),
                Var('v'),
            ),
            abstraction(
                'u',
                apply(
                    Var('u'),
                    Var('v'),
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
        apply(
            abstraction(
                'x',
                apply(
                    apply(Var('x'), Var('x')),
                    Var('y')
                )
            ),
            abstraction(
                'y',
                apply(Var('y'), Var('z'))
            ),
        );

    let y = x.to_normal_form();
    let real = apply(
        apply(Var('z'), Var('z')),
        Var('y'));

    assert!(y == real);
}

#[test]
fn e() {
    println!("(λx.λy.xyy)(λu.uyx)");
    println!("(λα.αyxα)");
    let x =
        apply(
            abstraction(
                'x',
                abstraction(
                    'y',
                    apply(
                        apply(
                            Var('x'),
                            Var('y')),
                        Var('y')),
            )),
            abstraction(
                'u',
                apply(
                    apply(
                        Var('u'), Var('y')),
                    Var('x'))));
    let y = x.to_normal_form();
    let real = 
        abstraction(
            'α',
            apply(
                apply(
                    apply(
                        Var('α'),
                        Var('y')),
                    Var('x')),
                Var('α')));

    dbg!(&y);
    dbg!(&real);
    assert!(real == y);
}

#[test]
fn f() {
    dbg!("λx.λy.λz.xz(yz)");
    let a =
        abstraction(
            'x',
            abstraction(
                'y',
                abstraction(
                    'z',
                    apply(
                        apply(
                            Var('x'),
                            Var('z'),
                        ),
                        apply(
                            Var('y'),
                            Var('z'),
                        )))));

    dbg!("(λx.λy.yx)u");
    let b =
        apply(swap(),
            Var('u')).to_normal_form();

    dbg!(&b);
    println!("***FINISHED B***");
    assert!(b == abstraction('y', apply(Var('y'), Var('u'))));

    let c =
        apply(
            swap(),
            Var('x')).to_normal_form();

    dbg!(&c);
    println!("***FINISHED C***");
    assert!(c == abstraction('y', apply(Var('y'), Var('x'))));

    let d = Var('w');
    println!("***FINISHED D***");

    let combined =
        apply(apply(apply(a, b), c), d).to_normal_form();
    //let ab = apply(a, b).to_normal_form();
    //dbg!(&ab);

    //println!("***Applied B to A***");


    let real =
        apply(apply(Var('w'), Var('u')), 
              apply(Var('w'), Var('v')));

    dbg!(&combined, &real);
    assert!(combined == real);

}

