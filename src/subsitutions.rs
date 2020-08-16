use std::fmt;
use crate::terms::Term;

pub struct Substitution {
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

impl fmt::Display for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} -> {}]", self.to_replace, self.replace_with)
    }
}
