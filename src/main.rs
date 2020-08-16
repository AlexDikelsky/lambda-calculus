use std::fmt;

use crate::Term::Var;
use crate::Term::Abs;
use crate::Term::App;

fn main() {
    let x = Abs('a', Box::new(Var('a')));
    let y = Abs('b', Box::new(Var('b')));
    let f = App(Box::new(x), Box::new(y));
    println!("{}", f.naieve_rewrite());
    println!("{}", App(Box::new(f.clone()), Box::new(f.clone())).naieve_rewrite());
}

enum Term {
    Var(char),
    Abs(char, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Term {
    pub fn naieve_rewrite(&self) -> String {
        match &self {
            Term::Var(c)      => format!("{}", c),
            Term::Abs(c, a)   => format!("(λ{}.{})", c, a.naieve_rewrite()),
            Term::App(a, b)   => format!("({} {})", a.naieve_rewrite(), b.naieve_rewrite()),
        }
    }
    pub fn clone(&self) -> Self {
        match &self {
            Term::Var(c)      => Var(*c),
            Term::Abs(c, a)   => Abs(*c, Box::new((*a).clone())),
            Term::App(a, b)   => App(Box::new((*a).clone()), Box::new((*b).clone())),
        }
    }

    //pub fn de_brujin(&self, index: usize) -> String {
    //    match &self {
    //        Term::Var(n)    => format!("{}", n),
    //        Term::Abs(a)    => format!("(λ.{})", a.de_brujin(index)),
    //        Term::App(a, b) => format!("({} {})", a.de_brujin(index), b.de_brujin(index)),
    //    }
    //}
    //pub fn single_var_to_string(&self) -> String {
    //    match &self {
    //        Term::Var(n)    => "x".to_string(),
    //        Term::Abs(a)    => format!("(λ{}.{})", a.single_var_to_string(), a.single_var_to_string()),
    //        Term::App(a, b) => format!("({} {})", a.single_var_to_string(), b.single_var_to_string()),
    //    }
    //}
}


//impl fmt::Display for LatLonPoint {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "Lat: {0:.5}, Lon: {1:.5}", self.phi, self.lambda)
//    }
//}

