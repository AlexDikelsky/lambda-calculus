//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.
//
//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY of FITNESS FOR A PARTICULAR PURPOSE. See the
//GNU General Public License for more details.
//
//You should have recieved a copy of the GNU General Public License
//along with this program. If not, see <https://www.gnu.org/licenses/>
use crate::terms::Term::Var;
use crate::terms::Term::Abs;
use crate::terms::Term::App;
use crate::terms::Term;

use crate::combinators::bool_combinators::{and, or, not};

use crate::aux::apply;
use crate::aux::abstraction;


// Incorrect xor operator
// λa.λb. or (and a b) (not (and a b))
pub fn wrong_xor() -> Term {
    let aandb = apply(apply(and(), Var('a')), Var('b'));
    
    abstraction('a',
        abstraction('b',
            apply(apply(or(), 
                        aandb.clone()), 
                  apply(not(), aandb))))

}

