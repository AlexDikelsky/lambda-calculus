use std::fmt;

use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::letters::LETTERS;

// Note that derived partialeq will say
// λx.x != λy.y
#[derive(PartialEq)]
pub enum Term {
    Var(char),
    Abs(char, Box<Term>),
    App(Box<Term>, Box<Term>),
}


impl Term {
    pub fn print(&self) -> String {
        match &self {
            Term::Var(c)      => format!("{}", c),
            Term::Abs(c, a)   => format!("(λ{}.{})", c, a.print()),
            Term::App(a, b)   => format!("({} {})", a.print(), b.print()),
        }
    }

    pub fn print_debug(&self) -> String {

        // Paren stack will always be balanced, so don't need to check
        // that this number is greater than 0
        let mut depth = 0;
        let n_tabs = |x| (0..x).fold("".to_string(), |acc, prev| acc + "  ");

        self.print().chars().map(|c| {
            match c {
                '(' => { depth += 1; "(\n".to_string() + &n_tabs(depth) },
                ')' => { depth -= 1; "\n".to_string() + &n_tabs(depth) + ")" },
                a => a.to_string(),
            }
        }).collect()
    }



    pub fn apply_abs(self, replace_with: Term) -> Self {
        println!("Evaluing {}{}", &self, &replace_with);

        match self {

            // Normal forms
            Term::Var(c)      => self,
            Term::App(_, _)   => App(Box::new(self), Box::new(replace_with)),

            // Keep applying
            Term::Abs(c, a)   => a.substitue(Substitution { to_replace: c, replace_with: Box::new(replace_with) }),
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
        println!("Substitg {} in {}", &sub, &self);

        let x = match self {
            Term::Var(c)      => if c == sub.to_replace { *sub.replace_with } else { Var(c) },

            Term::Abs(c, a)   => {
                //dbg!(&a, sub.to_replace, c, a.free_vars());
                //dbg!(a.free_vars().contains(&c));

                match sub.to_replace == c {
                    // Stop if [x -> s]λx.(anything) because overridden by scope
                    true => Abs(c, a),

                    false  => match sub.replace_with.free_vars().contains(&c) {
                        // If a variable you're substituting with will get "captured"
                        // by the current abstraction variable when substituted it in, 
                        // replace the abstraction variable with a Greek letter
                        true => Abs(c, Box::new(a.substitue(sub))),

                        // Otherwise, continue as normal
                        false => Abs(c, Box::new(a.substitue(sub))),
                    }
                }
            },

            Term::App(a, b)   => App(Box::new(a.substitue(sub.clone())), 
                                     Box::new(b.substitue(sub))),
        };
        println!("Result: {}", &x);
        x
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}
impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}
