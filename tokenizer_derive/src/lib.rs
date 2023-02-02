//! The macro which converts a struct or tuple into one which is able to be scraped easily.
//!
//! An example of this would be here:
//! ```rust
//! #[derive(Scraper)]
//! pub struct RedditListItem {
//!     #[scrape(xpath = r#"//a[@data-click-id="body"]/@href"#)]
//!     pub urls: Vec<String>
//! }
//! ```

#[macro_use] extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use quote::{__private::Span, ToTokens};
use symbol::Symbol;
use syn::{Attribute, Data, DeriveInput, ExprAssign, Fields, Meta, NestedMeta, spanned::Spanned, Path, Result, punctuated::Punctuated, Variant, token::{Comma, Enum}, Ident, parse::{ParseStream, Parse, Parser}};

mod symbol;


// https://doc.rust-lang.org/reference/procedural-macros.html

/// The macro which converts a struct or tuple into one which is able to be scraped easily.
///
/// An example of this would be here:
/// ```rust
/// #[derive(Scraper)]
/// pub struct RedditListItem {
///     #[scrape(xpath = r#"//a[@data-click-id="body"]/@href"#)]
///     pub urls: Vec<String>
/// }
/// ```
#[proc_macro_derive(Tokenizer, attributes(token))]
pub fn derive_tokenizer(input: TokenStream) -> TokenStream 
{
	let mut input = parse_macro_input!(input as DeriveInput);
	//let body = define_body(&mut input.data);
	let name = input.ident;
	let mut defs:  Vec<(&Ident, &Ident, Def)> = vec![];
	//println!("Текущий тип токена: {name}");
	match &mut input.data
	{
		Data::Struct(_) =>unimplemented!("Struct"),
		Data::Union(_) => unimplemented!("Union"),
		Data::Enum(e) => 
		{
			for var in  &e.variants
			{
				if var.attrs.len() > 0
				{
					let tt = Def::new(var.span(), &var.attrs);
					//println!("{}::{}", &name.to_string(), var.ident.to_string());
					//println!("{}", tt.pattern.as_ref().map_or("нет значения",|v|v));
					//println!("{}", tt.precedence.as_ref().map_or("нет значения",|v|v));
					//println!("{}", tt.converter.as_ref().map_or("нет значения",|v|v));
					defs.push((&name, &var.ident, tt));
				}
			};
		}
	}
	return quote!(
		impl<T> Definitions<T> for TokenDefinition<T> where T: std::clone::Clone
        {
			fn get_defs() -> Vec<TokenDefinition<T>>
            {
				let mut v : Vec<TokenDefinition<T>> = vec![];
				for d in defs
				{
					let enu = [d.0.to_string(), "::".to_owned(), d.1.to_string()].concat();
					let pattern = d.2.pattern.as_ref().unwrap();
					let conv: Option<(String, String)> = d.2.split_conv();
					let pr : u8 = d.2.get_precendence();
					let t = TokenDefinition::<T>::new(enu, pattern, conv.map_or(None, |p|p &[&p.0, &p.1]));
					v.push(t);
				}
				//let t = TokenDefinition::<T>new(t: )
				//new(return_token : T, regex_pattern : &str, precedence : u8, converter : Option<[&str; 2]>) -> Result<TokenDefinition<T>, Error>
				v
			}
		}
	).into();
	
	
}

struct Def
{
	span: Span,
	converter: Option<String>,
	pattern: Option<String>,
	precedence: Option<String>
}

impl Def
{
	pub fn new(span: Span, attributes: &[Attribute]) -> Self 
    {
		let pattern = get_attr_value(symbol::PATTERN, attributes);
		let precedence = get_attr_value(symbol::PRECEDENCE, attributes);
		let converter = get_attr_value(symbol::CONVERTER, attributes);
		Self 
        {
			span,
			converter,
			pattern,
			precedence
		}
	}
	pub fn get_precendence(&self) -> u8
	{
	   if let Some(p) = self.precedence.as_ref()
	   {
			let p= p.parse::<u8>();
			if p.is_err()
			{
				eprint!("Ошибка значения очередности precedence {} - значение является типом u8", p.err().unwrap());
				return 0;
			}
			else 
			{
				return p.unwrap();
			}
	   }
	   0
	}
	pub fn split_conv(&self) -> Option<(String, String)>
	{
		let conv = self.converter.as_ref()?;
		if !conv.contains(">")
		{
			eprint!("Ошибка, неправильная конструкция в конвертере значений! {} - должна быть: изменяемое>измененное", conv);
			return None;
		}
		let parsed: Vec<&str> = conv.split(">").collect();
		let first = parsed.iter().nth(0)?;
		let second = parsed.iter().nth(1)?;
		return Some((first.to_string(), second.to_string()));
	}

}

///TODO до сюда вроде все понятно) но только снизу

///Возвращает только значение аттрибута
fn get_attr_value(attr_name: Symbol, attributes: &[Attribute]) -> Option<String> 
{
	for attr in attributes 
    {
		if attr.path == symbol::BASE 
        {
			let parsed = parse_attr(attr)?;
			if parsed.0 == attr_name 
            {
				return Some(parsed.1);
			}
		}
	}
	None
}


fn parse_attr(attr: &Attribute) -> Option<(Path, String)> 
{
	let stream = attr.parse_args::<ExprAssign>().ok()?;
	let left = if let syn::Expr::Path(value) = *stream.left 
    {
		value
	} 
    else 
    {
		return None;
	};
	let right = if let syn::Expr::Lit(value) = *stream.right 
    {
		value
	} 
    else 
    {
		return None;
	};

	let right_value = if let syn::Lit::Str(value) = right.lit 
    {
		value.value()
	} 
    else 
    {
		return None;
	};
	Some((left.path, right_value))
}

fn parse_attr_name(attr: &Attribute) -> Option<Path> 
{
	// TODO: Actually use parse_meta() for all attributes instead of just this one.

	let parse = attr.parse_meta().expect("--------------------------------------------");
	if let Meta::List(val) = parse 
    {
		let ret = val.nested.into_iter().next();
		if let NestedMeta::Meta(Meta::Path(path)) = ret? 
        {
			return Some(path);
		}
	}
	None
}
