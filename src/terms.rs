use std::fmt;

use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::letter_list::LETTERS;
use crate::constants::var_x;

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
            Term::Abs(c, a)   => format!("[λ{}.{}]", c, a.print()),
            Term::App(a, b)   => format!("({} {})", a.print(), b.print()),
        }
    }

    pub fn apply_abs(self, replace_with: Term) -> Self {
        println!("Evaluing {}{}", &self, &replace_with);

        match self {

            // Normal forms
            Term::Var(c)      => self,
            Term::App(_, _)   => App(Box::new(self), Box::new(replace_with)),

            // Keep applying
            Term::Abs(c, a)   => a.substitue(Substitution { to_replace: c, replace_with: replace_with }),
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
            Term::Var(c)      => if c == sub.to_replace { sub.replace_with } else { Var(c) },

            Term::Abs(c, a)   => {
                match sub.to_replace == c {
                    // Stop if [x -> s]λx.(anything) because overridden by scope
                    true => Abs(c, a),

                    false  => match sub.replace_with.free_vars().contains(&c) {
                        // If the term you're substituting with will get "captured"
                        // by the current abstraction variable when substituted it in, 
                        // replace the abstraction variable with a Greek letter

                        true => {
                            let new_letter = a.next_unused_var_name();
                            Abs(new_letter,
                                Box::new(a.replace_var_name(c, new_letter)))
                        },

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

    fn get_all_var_names(&self) -> Vec<char> {
        match &self {
            Term::Var(c) => vec![*c],
            Term::Abs(c, a) => vec![*c].into_iter()
                .chain(a.get_all_var_names().into_iter()).collect(),
            Term::App(a, b) => a.get_all_var_names().into_iter()
                .chain(b.get_all_var_names().into_iter()).collect(),
        }
    }

    fn next_unused_var_name(&self) -> char {
        *(LETTERS.iter().skip_while(|c| self.get_all_var_names().contains(c)).next()
            .expect("Too many letters used, this was a dumb idea anyway"))
    }

    fn replace_var_name(self, old_letter: char, new_letter: char) -> Self {
        match self {
            Term::Var(c) => match c == new_letter { 
                
                true  => panic!("I thought we agreed not to use Greek letters"),
                false => match c == old_letter {
                    true  => Var(new_letter),
                    false => Var(c),
                }
            },

            Term::Abs(c, a) => match c == new_letter {
                true  => panic!("I thought we agreed not to use Greek letters"),
                false => match c == old_letter {
                    true  => Abs(new_letter, Box::new(a.replace_var_name(old_letter, new_letter))),
                    false => Abs(c, Box::new(a.replace_var_name(old_letter, new_letter))),
                }
            }

            Term::App(a, b) => App(
                Box::new(a.replace_var_name(old_letter, new_letter)), 
                Box::new(b.replace_var_name(old_letter, new_letter))),
        }
    }


    pub fn to_normal_form(self) -> Self {
        println!("To normal form {}", &self);
        match self {
            // Entire form is one variable
            Term::Var(_)    => self,


            // case where lambda but not applied to anything,
            //  Keep β reducing, but don't need to use this abstraction
            Term::Abs(_, a) => a.to_normal_form(),


            //    Application
            // Var, Var => normal form, return self
            // Var, Abs => normal form, return self
            // Var, App => Keep looking in App
            //
            // Abs, Var => Substitute Var -> Abs(c)
            // Abs, Abs => Substitute Abs2 -> Abs1(c)
            // Abs, App => Substitute App -> Abs(c)
            //
            // App, Var => Keep looking in App
            // App, Abs => Keep looking in both
            // App, App => Keep looking in both
            Term::App(a, b) => match (*a, *b) {
                (Var(c1), Var(c2)) => App(Box::new(Var(c1)), 
                                          Box::new(Var(c2))),
                (Var(c1), Abs(abs_c, abs_term)) =>
                    App(Box::new(Var(c1)),
                        Box::new(Abs(abs_c, abs_term))),

                (Var(c1), App(a1, a2)) =>
                    App(Box::new(Var(c1)),
                        Box::new(App(Box::new(a1.to_normal_form()),
                                     Box::new(a2.to_normal_form())))),


                //All of these are some variation on substitute with the second term
                // I think with an inner match this could be simplified
                (Abs(c1, inner_abs), Var(c2)) => Abs(c1, inner_abs) // abstraction term
                    .strict_substitute(Substitution { 
                        to_replace: c1,
                        replace_with: Var(c2),
                    }).to_normal_form(),

                (Abs(c1, inner_abs1), Abs(c2, inner_abs2)) => Abs(c1, inner_abs1)
                    .strict_substitute(Substitution {
                        to_replace: c1,
                        replace_with: Abs(c2, inner_abs2),
                    }).to_normal_form(),

                (Abs(c1, inner_abs), App(a1, a2)) => Abs(c1, inner_abs)
                    .strict_substitute(Substitution {
                        to_replace: c1,
                        replace_with: App(a1, a2)
                    }).to_normal_form(),

                (App(a1, a2), Var(c2)) =>
                    App(Box::new(
                        App(
                            Box::new(a1.to_normal_form()),
                            Box::new(a2.to_normal_form())
                        )), 
                        Box::new(Var(c2))),

                (App(a1, a2), Abs(c2, abs2)) =>
                    App(Box::new(
                        App(
                            Box::new(a1.to_normal_form()),
                            Box::new(a2.to_normal_form())
                        )), 
                        Box::new(
                            Abs(c2, abs2))),

                (App(a1, a2), App(b1, b2)) =>
                    App(
                        Box::new(App(
                            Box::new(a1.to_normal_form()),
                            Box::new(a2.to_normal_form())
                        )), 
                        Box::new(App(
                            Box::new(b1.to_normal_form()),
                            Box::new(b2.to_normal_form())
                        ))),
                
            },
        }
    }


    fn strict_substitute(self, sub: Substitution) -> Self {
        println!("Substitg {} in {}", &sub, &self);
        let saved_copy = match self {
            Term::Var(c) => match sub.to_replace == c {
                true  => sub.replace_with,
                false => Var(c),
            },

            Term::App(a, b) => match *a {
                //Done with first, but check next value
                Term::Var(char_in_var) => App(
                    Box::new(Var(char_in_var)), 
                    Box::new(b.strict_substitute(sub))),

                //something like (λa.a)b, substitue b for the bounds var of a
                // Need to put logic for fixing names here,
                Term::Abs(inner_abs_var, abstration_term) => 
                    match sub.replace_with.free_vars().contains(&inner_abs_var) {

                        //Need to change inner vars, then keep substituting
                        true => {
                            let new_letter = abstration_term.next_unused_var_name();
                            abstration_term.replace_var_name(inner_abs_var, new_letter)
                                .strict_substitute(Substitution {
                                    to_replace: new_letter,
                                    replace_with: *b,
                                })
                        },

                        //Don't need to change inner vars
                        // Keep substituting
                        false => abstration_term.strict_substitute(Substitution {
                            to_replace: inner_abs_var,
                            replace_with: *b,
                        }),
                    }

                Term::App(x, y) => App(
                    Box::new(x.strict_substitute(sub.clone())),
                    Box::new(y.strict_substitute(sub))),
            }

            // Don't need to check for FV in sub.with
            // Also keep evaluataing even if current letter = sub.for
            Term::Abs(c, a) => a.strict_substitute(sub),

        };
        println!("Result: {}", &saved_copy);
        saved_copy
    }
        



                    



}

//impl PartialEq for Term {
//    fn eq(&self, other: &Self) -> bool {
//
//    }
//}

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
