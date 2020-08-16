use std::fmt;

use crate::Term::Var;
use crate::Term::Abs;
use crate::Term::App;

fn main() {
    let y = Abs(Box::new(|x| Var));
    println!("{}", y.to_string());
}

enum Term {
    Var,
    Abs(Box<dyn Fn(Term) -> Term>),
    App(Box<dyn Fn(Term) -> Term>, Box<dyn Fn(Term) -> Term>),
}

impl Term {
    pub fn to_string(&self) -> String {
        match &self {
            Term::Var       => "x".to_string(),
            Term::Abs(a)    => "λ.".to_string() + a.to_string(),
            Term::App(a, b) => "y y".to_string(),
        }
    }
}


//impl fmt::Display for LatLonPoint {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "Lat: {0:.5}, Lon: {1:.5}", self.phi, self.lambda)
//    }
//}

