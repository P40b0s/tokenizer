use std::fmt::{self, Display};
use syn::{Ident, Path};


pub const BASE: Symbol = Symbol("token");
pub const PATTERN: Symbol = Symbol("pattern");
pub const PRECEDENCE: Symbol = Symbol("precedence");
pub const CONVERTER: Symbol =  Symbol("converter");

// From Serde Symbol
#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

impl PartialEq<Symbol> for Ident 
{
	fn eq(&self, word: &Symbol) -> bool 
    {
		self == word.0
	}
}

impl<'a> PartialEq<Symbol> for &'a Ident 
{
	fn eq(&self, word: &Symbol) -> bool 
    {
		*self == word.0
	}
}

impl PartialEq<Symbol> for Path 
{
	fn eq(&self, word: &Symbol) -> bool 
    {
		self.is_ident(word.0)
	}
}

impl<'a> PartialEq<Symbol> for &'a Path 
{
	fn eq(&self, word: &Symbol) -> bool 
    {
		self.is_ident(word.0)
	}
}

impl Display for Symbol 
{
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result 
    {
		formatter.write_str(self.0)
	}
}