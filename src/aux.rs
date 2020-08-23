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
use crate::terms::Term;
use crate::terms::Term::App;
use crate::terms::Term::Abs;
use crate::lambda::TermParser;
use crate::terms;

pub fn apply(a: Term, b: Term) -> Term {
    App(Box::new(a), Box::new(b))
}

pub fn abstraction(c: char, b: Term) -> Term {
    Abs(c, Box::new(b))
}


pub fn parse(s: &str) -> Result<terms::Term, lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>, &'static str>> {
    TermParser::new().parse(s)
}
