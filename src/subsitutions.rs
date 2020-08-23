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
use std::fmt;
use crate::terms::Term;

pub struct Substitution {
    pub to_replace: char,
    pub replace_with: Term,
    pub debug: bool,
}

impl Substitution {
    pub fn clone(&self) -> Self {
        Substitution { 
            to_replace: self.to_replace,
            replace_with: self.replace_with.clone(),
            debug: self.debug,
        }
    }
}

impl fmt::Display for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} -> {}]", self.to_replace, self.replace_with)
    }
}
