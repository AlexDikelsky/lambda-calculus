use std::fmt;

use crate::Term::Var;
use crate::Term::Abs;
use crate::Term::App;

fn main() {
    let id = Abs('x', Box::new(Var('x')));
    let tru = Abs('y', Box::new(Abs('z', Box::new(Var('y')))));
    let fls = Abs('a', Box::new(Abs('b', Box::new(Var('b')))));

    let and = Abs('c', Box::new(
                Abs('d', Box::new(
                  App(Box::new(App(Box::new(Var('c')), Box::new(Var('d')))), Box::new(fls.clone()))))));

    let a = and.apply_abs(tru.clone());
    dbg!(a);
    //println!("{} and {} = {}", tru.clone(), fls.clone(), and.apply_abs(tru).apply_abs(fls));

}

// Note that derived partialeq will say
// λx.x != λy.y
#[derive(PartialEq)]
enum Term {
    Var(char),
    Abs(char, Box<Term>),
    App(Box<Term>, Box<Term>),
}

struct Substitution {
    pub to_replace: char,
    pub replace_with: Box<Term>,
}

impl Substitution {
    pub fn clone(&self) -> Self {
        Substitution { 
            to_replace: self.to_replace,
            replace_with: Box::new(self.replace_with.clone()) }
    }
}

impl Term {
    pub fn naieve_print(&self) -> String {
        match &self {
            Term::Var(c)      => format!("{}", c),
            Term::Abs(c, a)   => format!("(λ{}.{})", c, a.naieve_print()),
            Term::App(a, b)   => format!("({} {})", a.naieve_print(), b.naieve_print()),
        }
    }

    pub fn apply_abs(self, replace_with: Term) -> Self {
        println!("Applying {} to {}", Substitution { to_replace: match &self { 
            Term::Var(c) => *c,
            Term::Abs(c, _) => *c,
            _ => '0',
        }, replace_with: Box::new(replace_with.clone()) }, &self);

        match self {
            Term::Var(c)      => Var(c),
            Term::Abs(c, a)   => a.substitue(Substitution { to_replace: c, replace_with: Box::new(replace_with) }),
            Term::App(a, b)   => App(a, b),
        }
    }

    pub fn clone(&self) -> Self {
        match &self {
            Term::Var(c)      => Var(*c),
            Term::Abs(c, a)   => Abs(*c, Box::new((*a).clone())),
            Term::App(a, b)   => App(Box::new((*a).clone()), Box::new((*b).clone())),
        }
    }

    //Change this to a set later, and use union rather than plus
    //and subtraction rather than filter
    pub fn free_vars(&self) -> Vec<char> {
        match &self {
            Term::Var(c)      => vec![*c],
            Term::Abs(c, a)   => a.free_vars().into_iter().filter(|x| *x != *c).collect(),
            Term::App(a, b)   => a.free_vars().into_iter()
                .chain(b.free_vars().into_iter()).collect(),
        }
    }

    pub fn substitue(self, sub: Substitution) -> Self {
        println!("Substituting {} in {}", Substitution { to_replace: match &self { 
            Term::Var(c) => *c,
            Term::Abs(c, _) => *c,
            _ => '0',
        }, replace_with: Box::new(sub.replace_with.clone()) }, &self);

        match self {
            Term::Var(c)      => if c == sub.to_replace { *sub.replace_with } else { Var(c) },

            Term::Abs(c, a)   => match sub.to_replace != c && 
                                        !a.free_vars().contains(&c) {
                true  => Abs(c, Box::new(a.substitue(sub))),
                false => Abs(c, a),
            },
            Term::App(a, b)   => App(Box::new(a.substitue(sub.clone())), 
                                     Box::new(b.substitue(sub))),
        }
    }
}


impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.naieve_print())
    }
}
impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.naieve_print())
    }
}
impl fmt::Display for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} -> {}]", self.to_replace, self.replace_with)
    }
}

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
    let x = App(Box::new(ident_a.clone()), Box::new(Var('e'))).substitue(
            Substitution {
                to_replace: 'e',
                replace_with: Box::new(Var('n')),
            });
    let y = App(Box::new(ident_a.clone()), Box::new(Var('n')));
    println!("{}, {}", x, y);
    assert!(x == y);
    
    //assert!(
    //    ident_a.clone().substitue(
    //        Substitution {
    //            to_replace: 'a',
    //            replace_with: Box::new(Var('b')),
    //        }) == ident_b.clone());

}

