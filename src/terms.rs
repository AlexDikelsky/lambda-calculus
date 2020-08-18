use std::fmt;

use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::subsitutions::Substitution;
use crate::letter_list::LETTERS;
use crate::aux::apply;
use crate::aux::abstraction;

pub enum Term {
    Var(char),
    Abs(char, Box<Term>),
    App(Box<Term>, Box<Term>),
}


impl Term {
    pub fn print(&self) -> String {
        match &self {
            Var(c)      => format!("{}", c),
            Abs(c, a)   => format!("[λ{}.{}]", c, a.print()),
            App(a, b)   => format!("({} {})", a.print(), b.print()),
        }
    }

    pub fn apply_abs(self, replace_with: Term) -> Self {
        println!("Evaluing {}{}", &self, &replace_with);

        match self {

            // Normal forms
            Var(c)      => self,
            App(_, _)   => apply(self, replace_with),

            // Keep applying
            Abs(c, a)   => a.substitue(Substitution { to_replace: c, replace_with: replace_with, debug: true }),
        }
    }

    pub fn clone(&self) -> Self {
        match &self {
            Var(c)      => Var(*c),
            Abs(c, a)   => abstraction(*c, (*a).clone()),
            App(a, b)   => apply((*a).clone(), (*b).clone()),
        }
    }

    //Change this to a set later, and use union rather than plus
    //and subtraction rather than filter
    pub fn free_vars(&self) -> Vec<char> {
        match &self {
            Var(c)      => vec![*c],
            Abs(c, a)   => a.free_vars().into_iter().filter(|x| *x != *c).collect(),
            App(a, b)   => a.free_vars().into_iter()
                .chain(b.free_vars().into_iter()).collect(),
        }
    }

    pub fn substitue(self, sub: Substitution) -> Self {
        println!("Substitg {} in {}", &sub, &self);

        let x = match self {
            Var(c)      => if c == sub.to_replace { sub.replace_with } else { Var(c) },

            Abs(c, a)   => {
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
            Var(c) => vec![*c],
            Abs(c, a) => vec![*c].into_iter()
                .chain(a.get_all_var_names().into_iter()).collect(),
            App(a, b) => a.get_all_var_names().into_iter()
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
                     App(a, b) => format!("{}, {}", a.is_normal_form(), &b.is_normal_form()),
                     Abs(a, b) => format!("{}", b.is_normal_form()),
                 });

        println!("To normal form {}, self = {}, lower = {}", &self, dbg_strings.0, dbg_strings.1);
        //println!("To normal {}", &self);

        let temp = match self {
            // Entire form is one variable
            Var(_)    => self,


            // case where lambda but not applied to anything,
            //  Keep β reducing, but don't need to use this abstraction
            //  Make sure to stay an abstraction though
            Abs(c, a) => abstraction(c, a.to_normal_form()),


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

            App(a, b) => match (a.is_normal_form(), b.is_normal_form()) {
                (true,  true ) => {
                    match *a {
                        // If the first term is an abstraction, then it can be reduced again
                        //   This means that the first thing shouldn't be the same
                        //   abstraction anymore
                        Abs(c, inner_abs) => inner_abs.strict_substitute(Substitution {
                            to_replace: c,
                            replace_with: *b,
                            debug: true,
                        }).to_normal_form(),

                        // Otherwise, return same thing because both are normal
                        _ => { 
                            //assert!(a.clone().to_normal_form() == *a, "math failed");
                            //assert!(b.clone().to_normal_form() == *b, "math failed");
                            //let p = apply(a.clone(), b.clone());
                            //assert!(p.clone().to_normal_form() == p, "math failed");
                            App(a, b)
                        },
                    }
                }

                // First one might be an abstraction
                (true,  false) => match *a {
                    // Replace the term in the abstraction with b
                    Abs(c, inner_abs) => inner_abs.strict_substitute(Substitution {
                        to_replace: c,
                        replace_with: b.to_normal_form(),
                        debug: true,
                    }),
                    _ => apply(*a, b.to_normal_form())
                },
                (false, true ) => match *a {
                    //Substitute b for a's bound
                    //   Use B as the abstraction to replace for c
                    Abs(c, inner_abs) => inner_abs.strict_substitute(Substitution {
                        to_replace: c,
                        replace_with: *b,
                        debug: true,
                    }),

                    // First thing could become an abstraction, so you need to
                    // keep checking
                    App(_, _) => apply(a.to_normal_form(), *b).to_normal_form(),
                    Var(_) => panic!("Var treated as non-normal"),
                },

                // Can't say much here, but need to normalize the result because of cases like
                //    (λa.(λx.x)c))((λa.(λx.x)c)
                (false, false) => apply(a.to_normal_form(), b.to_normal_form()).to_normal_form(),
            }
        };
        println!("Now is {}, self_type = {}, lower = {}", &temp, dbg_strings.0, dbg_strings.1);
        //println!("Now is {}", &temp);
        temp
    }


    fn strict_substitute(self, sub: Substitution) -> Self {
        let dbg_str = match &self {
                     Var(_) => "Var",
                     App(_, _) => "App",
                     Abs(_, _) => "Abs",
        };
        let s = sub.debug;
        if s {
            println!("Substitg {} in {}, type {}", &sub, &self, dbg_str);
            //println!("Substitg {} in {}", &sub, &self);
        }
        let saved_copy = match self {
            Var(c) => match sub.to_replace == c {
                true  => sub.replace_with,
                false => Var(c),
            },

            App(a, b) => apply(
                a.strict_substitute(sub.clone()),
                b.strict_substitute(sub),
                ),

            Abs(c, a) => match sub.replace_with.free_vars().contains(&c) {

                // Var would be captured
                true  => {
                    // This works by swapping the bounds of the current abstraction
                    // to a new value, then continuing the substitution as normal
                    let new_letter = a.next_unused_var_name();
                    println!("Swapping bound var {} with {}", c, new_letter);
                    abstraction(
                        new_letter, 
                        a.strict_substitute(Substitution {
                            to_replace: c,
                            replace_with: Var(new_letter),
                            debug: sub.debug,
                        }).strict_substitute(sub))
                },

                false => match c == sub.to_replace {
                    // Stop substituting because tighter binding scope
                    //  Still need to normalize, but not here
                    true  => abstraction(c, *a),

                    // Some other variable, so still in scope and
                    // isn't at risk of capturing. Keep replacing here
                    false => abstraction(c, a.strict_substitute(sub)),
                }
            },

        };
        if s {
            println!("Result: {}, type was {}", &saved_copy, dbg_str)
        };

        saved_copy
    }
        
    fn to_reg_names(self) -> Self {
        match self {
            Var(_) => Var(self.next_unused_var_name()),
            Abs(c, a) => {
                let new_letter = a.next_unused_var_name();
                abstraction(new_letter, a.strict_substitute(Substitution {
                    to_replace: c,
                    replace_with: Var(new_letter),
                    debug: false,
                }))
            },
            App(a, b) => apply(a.to_reg_names(), b.to_reg_names()),
        }
    }

    pub fn equal_names_matter(self, other: Term) -> bool {
        match (self, other) {
            (Var(c1), Var(c2)) => c1 == c2,
            (Abs(c1, a1), Abs(c2, a2)) => (c1 == c2) && (a1.equal_names_matter(*a2)),
            (App(a1, b1) , App(a2, b2)) => a1.equal_names_matter(*a2) && b1.equal_names_matter(*b2),
            _ => false,
        }
    }

}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        //This could be much faster and memory intensive if you don't clone
        self.clone().to_reg_names().equal_names_matter(other.clone().to_reg_names())

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
