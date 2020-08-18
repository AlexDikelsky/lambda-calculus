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

    fn get_type(&self) -> String {
        match &self {
            Var(_) => "Var".to_string(),
            Abs(_, _) => "Abs".to_string(),
            App(_, _) => "App".to_string(),
        }
    }

    pub fn is_normal_form(&self) -> bool {
        match &self {
            Var(_) => true,
            Abs(_, a) => a.is_normal_form(),
            App(a, b) => match **a {
                Abs(_, _) => false,
                _ => a.is_normal_form() && b.is_normal_form(),
            }
        }
    }


    pub fn to_normal_form(self) -> Self {
        let dbg_strings = (self.get_type(), match &self {
                     Var(_) => "Var".to_string(),
                     App(a, b) => a.get_type() + &b.get_type(),
                     Abs(a, b) => b.get_type(),
                 });

        println!("To normal form {}, self_type = {}, lower = {}", &self, dbg_strings.0, dbg_strings.1);

        let temp = match self {
            // Entire form is one variable
            Term::Var(_)    => self,


            // case where lambda but not applied to anything,
            //  Keep β reducing, but don't need to use this abstraction
            //  Make sure to stay an abstraction though
            Term::Abs(c, a) => Abs(c, Box::new(a.to_normal_form())),


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

            Term::App(a, b) => match (a.is_normal_form(), b.is_normal_form()) {
                (true,  true ) => {
                    match *a {
                        // If the first term is an abstraction, then it can be reduced again
                        Abs(c, a1) => Abs(c, a1).strict_substitute(Substitution {
                            to_replace: c,
                            replace_with: *b,
                        }),

                        // Otherwise, return same thing
                        Var(_) => App(a, b),
                        App(_, _) => App(a, b),
                    }
                }
                (true,  false) => App(a, Box::new(b.to_normal_form())),
                (false, true ) => match *a {
                    //Glitch
                    Var(_) => panic!("Variable {} not recognized as normal", a),

                    //Substitute b for a's bound
                    Abs(c, a1) => Abs(c, a1).strict_substitute(Substitution {
                        to_replace: c,
                        replace_with: *b
                    }),

                    App(_, _) => a.to_normal_form(),
                },
                (false, false) => App(Box::new(a.to_normal_form()), 
                                      Box::new(b.to_normal_form())),
            }
            //Term::App(a, b) => match (*a, *b) {
            //    (Var(c1), Var(c2)) => App(Box::new(Var(c1)), 
            //                              Box::new(Var(c2))),
            //    (Var(c1), Abs(abs_c, abs_term)) =>
            //        App(Box::new(Var(c1)),
            //            Box::new(Abs(abs_c, abs_term))),

            //    (Var(c1), App(a1, a2)) =>
            //        App(Box::new(Var(c1)),
            //            Box::new(App(Box::new(a1.to_normal_form()),
            //                         Box::new(a2.to_normal_form())))),


            //    //All of these are some variation on substitute with the second term
            //    // I think with an inner match this could be simplified
            //    (Abs(c1, inner_abs), Var(c2)) => Abs(c1, inner_abs) // abstraction term
            //        .strict_substitute(Substitution { 
            //            to_replace: c1,
            //            replace_with: Var(c2),
            //        }),

            //    (Abs(c1, inner_abs1), Abs(c2, inner_abs2)) => Abs(c1, inner_abs1)
            //        .strict_substitute(Substitution {
            //            to_replace: c1,
            //            replace_with: Abs(c2, inner_abs2),
            //        }),

            //    (Abs(c1, inner_abs), App(a1, a2)) => Abs(c1, inner_abs)
            //        .strict_substitute(Substitution {
            //            to_replace: c1,
            //            replace_with: App(a1, a2)
            //        }),

            //    (App(a1, a2), Var(c2)) =>
            //        App(Box::new(
            //            App(
            //                a1,
            //                a2
            //            )),
            //            Box::new(Var(c2))).to_normal_form(),
            //        //App(Box::new(
            //        //    App(
            //        //        Box::new(a1.to_normal_form()),
            //        //        Box::new(a2.to_normal_form())
            //        //    ).to_normal_form()), 
            //        //    Box::new(Var(c2))),

            //    (App(a1, a2), Abs(c2, abs2)) =>
            //        App(Box::new(
            //            App(
            //                Box::new(a1.to_normal_form()),
            //                Box::new(a2.to_normal_form())
            //            )), 
            //            Box::new(
            //                Abs(c2, abs2))),

            //    (App(a1, a2), App(b1, b2)) =>
            //        App(
            //            Box::new(App(
            //                Box::new(a1.to_normal_form()),
            //                Box::new(a2.to_normal_form())
            //            )), 
            //            Box::new(App(
            //                Box::new(b1.to_normal_form()),
            //                Box::new(b2.to_normal_form())
            //            ))),
            //    
            //},
        };
        println!("Now is {}, self_type = {}, lower = {}", &temp, dbg_strings.0, dbg_strings.1);
        temp
    }


    fn strict_substitute(self, sub: Substitution) -> Self {
        let dbg_str = match &self {
                     Var(_) => "Var",
                     App(_, _) => "App",
                     Abs(_, _) => "Abs",
        };
        println!("Substitg {} in {}, type {}", &sub, &self, dbg_str);
        let saved_copy = match self {
            Term::Var(c) => match sub.to_replace == c {
                true  => sub.replace_with,
                false => Var(c),
            },

            Term::App(a, b) => App(
                Box::new(a.strict_substitute(sub.clone())),
                Box::new(b.strict_substitute(sub)),
                ).to_normal_form(),

            Term::Abs(c, a) => match sub.replace_with.free_vars().contains(&c) {

                // Possible capture, use π untill I can fix this
                true  => a.strict_substitute(Substitution {
                    to_replace: c,
                    replace_with: Var('π'),
                }).strict_substitute(sub),

                false => match c == sub.to_replace {
                    // Stop substituting because tighter binding scope
                    //  Still need to normalize
                    true  => a.strict_substitute(sub),

                    // Some other variable, so still in scope and
                    // isn't at risk of capturing. Keep replacing here
                    false => Abs(c, Box::new(a.strict_substitute(sub))),
                }
            },

        };
        println!("Result: {}, type was {}", &saved_copy, dbg_str);
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
