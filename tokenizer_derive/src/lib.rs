//! The macro compile TokensDefinitions for Lexer.
//!	Макрос для компиляции определений токенов TokensDefinitions для Lexer на основе атрибутов.
//! An example of this:
//! ```rust
//! #[derive(Tokenizer)]
//! pub enum TestTokens {
//! 	//pattern to recognize
//!     #[token(pattern = r#"/d+"#)]
//!     Token1,
//! 	#[token(pattern = r#"Aa+"#)]
//! 	//priority, in multiple matches will choise match with highter priority, 0 by default
//! 	#[token(precedence = "1")]
//! 	Token2,
//! 	#[token(pattern = r#"Bb+"#)]
//! 	#[token(precedence = "2")]
//! 	//Convert part before > to part after >
//! 	//Constriction *>Aa convert all matches to right part
//! 	#[token(converter = "Bb>Aa")]
//! 	Token3,
//! }
//! //Все определения токенов которые были определены аттрибутами для данного enum
//! let tokens_definitions = TestTokens::get_defs();
//! ```

#[macro_use] extern crate syn;
#[macro_use] extern crate quote;
extern crate proc_macro2;
use proc_macro::TokenStream;

//use proc_macro2::TokenTree;
use quote::{ToTokens};
use symbol::Symbol;
use syn::{
	Attribute,
	Data,
	DeriveInput,
	ExprAssign,
	//Fields,
	Meta,
	//NestedMeta,
//	spanned::Spanned,
	Path, punctuated::Punctuated, LitInt, Error, LitStr,
	//Result,
	//punctuated::Punctuated,
	//Variant,
//	token::{Comma, Enum},
	//Ident,
//	parse::{ParseStream, Parse, Parser}
};
mod symbol;


/// The macro compile TokensDefinitions for Lexer.
///
/// An example of this:
/// ```rust
/// #[derive(Tokenizer)]
/// pub enum TestTokens {
/// 	//pattern to recognize
///     #[token(pattern = r#"/d+"#)]
///     Token1,
/// 	#[token(pattern = r#"Aa+"#)]
/// 	//priority, in multiple matches will choise match with highter priority, 0 by default
/// 	#[token(precedence = "1")]
/// 	Token2,
/// 	#[token(pattern = r#"Bb+"#)]
/// 	#[token(precedence = "2")]
/// 	//Convert Bb to Aa
/// 	//Constriction *>Aa convert all matches to Aa
/// 	#[token(converter = "Bb>Aa")]
/// 	Token3,
/// }
/// 
/// //Все определения токенов которые были определены аттрибутами для данного enum
/// let tokens_definitions = TestTokens::get_defs();
/// ```
#[proc_macro_derive(Tokenizer, attributes(token))]
pub fn derive_tokenizer(inp: TokenStream) -> TokenStream 
{
	let mut input = parse_macro_input!(inp as DeriveInput);
	let name = input.ident;
	let mut arr: Vec<proc_macro2::TokenStream> = vec![];
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
					//let def = Def::new(&var.attrs);
					let defs = Def::new(&var.attrs);
					let enu = var.ident.clone();
					//println!("{:?}", &defs);
					for def in defs
					{
						
						let mut pattern: &String = &String::new();
						let mut pr = 0u8;
						if let Some(p) = def.pattern.as_ref()
						{
							pattern = p;
						}
						else
						{
							eprintln!("Вы не указали pattern для {}", &var.ident.to_string());
							return var.ident.to_token_stream().into();
						}
						if def.precedence.is_some()
						{
							pr = def.precedence.unwrap();
						}
						// if let Some(conv) = def.split_conv
						// {
						// 	let c1 = conv.0;
						// 	let c2 = conv.1;
						// 	let rr = quote!(::tokenizer::TokenDefinition::<#name>::new(#name::#enu, #pattern, #pr, Some([#c1, #c2])),);
						// 	arr.push(rr);
						// }
						if let Some(conv) = def.converter
						{
							let rr = quote!(::tokenizer::TokenDefinition::<#name>::new(#name::#enu, #pattern, #pr, Some(#conv.to_string())),);
							arr.push(rr);
						}
						else 
						{
							let rr = quote!(::tokenizer::TokenDefinition::<#name>::new(#name::#enu, #pattern, #pr, None),);
							arr.push(rr);
						}
						
						
					}
				}
			};
		}
	}
	return quote!(
		impl #name
        {
			///Получает все определения токенов, которые были определены в аттрибутах enum
			pub fn get_defs() -> Option<Vec<::tokenizer::TokenDefinition<#name>>>
            {
				let arr = [#(#arr)*].to_vec();
				let mut new = vec![];
				let mut error_bool : bool = false;
				for e in arr
				{
					if e.is_err()
					{
						error_bool = true;
						eprintln!("Ошибка в регексе! {}", &e.err().unwrap());
					}
					else
					{
						new.push(e.unwrap());
					}
				}
				if !error_bool
				{
					Some(new)
				}
				else
				{
					None
				}
			}
		}
	).into();
	
	
}


#[derive(Debug)]
struct Def
{
	//span: Span,
	converter: Option<String>,
	pattern: Option<String>,
	precedence: Option<u8>
}
impl Default for Def
{
	fn default() -> Self 
	{
		Def { converter: None, pattern: None, precedence: None }
	}
}

impl Def
{
	// pub fn new(attributes: &[Attribute]) -> Self 
    // {
	// 	let pattern = get_attr_value(symbol::PATTERN, attributes);
	// 	let precedence = get_attr_value(symbol::PRECEDENCE, attributes);
	// 	let converter = get_attr_value(symbol::CONVERTER, attributes);
	// 	Self 
    //     {
	// 		//span,
	// 		converter,
	// 		pattern,
	// 		precedence
	// 	}
	// }
	pub fn new(attributes: &[Attribute]) -> Vec<Def>
    {
		let defs = parse_tokens(attributes);
		if defs.is_ok()
		{
			return defs.unwrap();
		}
		else 
		{
			eprint!("Ошибка парсинга токенов: {}", defs.err().unwrap());
			return vec![];
		}
	}
	// pub fn get_precendence(&self) -> u8
	// {
	//    if let Some(p) = self.precedence.as_ref()
	//    {
	// 		let p= p.parse::<u8>();
	// 		if p.is_err()
	// 		{
	// 			eprint!("Ошибка значения очередности precedence {} - значение является типом u8", p.err().unwrap());
	// 			return 0;
	// 		}
	// 		else 
	// 		{
	// 			return p.unwrap();
	// 		}
	//    }
	//    0
	// }
	// pub fn split_conv(&self) -> Option<(String, String)>
	// {
	// 	let conv = self.converter.as_ref()?;
	// 	if !conv.contains(">")
	// 	{
	// 		eprint!("Ошибка, неправильная конструкция в конвертере значений! {} - должна быть: изменяемое>измененное", conv);
	// 		return None;
	// 	}
	// 	let parsed: Vec<&str> = conv.split(">").collect();
	// 	let first = parsed.iter().nth(0)?;
	// 	let second = parsed.iter().nth(1)?;
	// 	return Some((first.to_string(), second.to_string()));
	// }

}

///TODO до сюда вроде все понятно) но только снизу

///Возвращает только значение аттрибута
// fn get_attr_value(attr_name: Symbol, attributes: &[Attribute]) -> Option<String> 
// {
// 	for attr in attributes 
//     {
// 		if attr.path() == symbol::BASE 
//         {
// 			let parsed = parse_attr(attr)?;
// 			if parsed.0 == attr_name 
//             {
// 				return Some(parsed.1);
// 			}
// 		}
// 	}
// 	None
// }


// fn parse_attr(attr: &Attribute) -> Option<(Path, String)> 
// {
// 	let stream = attr.parse_args::<ExprAssign>().ok()?;
// 	let left = if let syn::Expr::Path(value) = *stream.left 
//     {
// 		value
// 	} 
//     else 
//     {
// 		return None;
// 	};
// 	let right = if let syn::Expr::Lit(value) = *stream.right 
//     {
// 		value
// 	} 
//     else 
//     {
// 		return None;
// 	};

// 	let right_value = if let syn::Lit::Str(value) = right.lit 
//     {
// 		value.value()
// 	} 
//     else 
//     {
// 		return None;
// 	};
// 	Some((left.path, right_value))
// }

fn parse_tokens(attributes: &[Attribute]) -> Result<Vec<Def>, syn::Error> 
{
	let mut defs: Vec<Def> = vec![];
	for attr in attributes
	{
		if attr.path().is_ident(&symbol::BASE)
		{
			let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
			{
				let mut def = Def::default();
				for meta in nested 
				{
					match meta 
					{
						// #[token(pattern())]
						Meta::List(meta) if meta.path.is_ident(&symbol::PATTERN) => 
						{
							let lit: LitStr = meta.parse_args()?;
							let p: String = lit.value();
							eprint!("значение паттерна: {}", &p);
							def.pattern = Some(p);
						},
						Meta::List(meta) if meta.path.is_ident(&symbol::PRECEDENCE) => 
						{
							let lit: LitInt = meta.parse_args()?;
							let p: u16 = lit.base10_parse()?;
							eprint!("значение очередности: {}", &p);
							def.precedence = Some(p as u8);
						},
						Meta::List(meta) if meta.path.is_ident(&symbol::CONVERTER) => 
						{
							let lit: LitStr = meta.parse_args()?;
							let p = lit.value();
							eprint!("значение конвертера: {}", &p);
							def.converter = Some(p);
						},
						_ => 
						{

							return Err(Error::new_spanned(meta, "нераспознана последовательность token"));
						}
					}
				}
				defs.push(def);
			}
		}
		else 
		{
			return Err(Error::new_spanned(attr.path(), "последовательность token не найдена"));
		}
	}
	Ok(defs)
}

// fn parse_attr_name(attr: &Attribute) -> Option<Path> 
// {
// 	// TODO: Actually use parse_meta() for all attributes instead of just this one.

// 	let parse = attr.parse.expect("--------------------------------------------");
// 	if let Meta::List(val) = parse 
//     {
// 		let ret = val.nested.into_iter().next();
// 		if let NestedMeta::Meta(Meta::Path(path)) = ret? 
//         {
// 			return Some(path);
// 		}
// 	}
// 	None
// }
