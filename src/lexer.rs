// use std::{collections::HashMap, ops::Deref};
// use crate::{token_definition::TokenDefinition, matches::TokenMatch, token::Token};
// use itertools::Itertools;

// pub trait Tokenizer<T>
// {
//     fn tokenize(&mut self, text : &str, def : Vec<TokenDefinition<T>>)-> Vec<Token<T>>;
// }

// pub struct Lexer<T>
// {
//     tokens : Vec<Token<T>>,
//     definitions : Vec<TokenDefinition<T>>,
// }
// impl<T> Lexer<T>
// {
//     fn find_matches(&self, text : &str) -> Vec<TokenMatch<T>>
//     {
//         let mut res : Vec<TokenMatch<T>> = Vec::new();
//         for def in self.definitions.iter()
//         {
//             def.find_matches(text).into_iter().for_each(|t| {
//                 res.push(t);
//             });
//         }
//         res
//     } 
//     pub fn sort_by_key_ref<A, F, K>(a: &mut [A], key: F) 
//     where
//     F: Fn(&A) -> &K,
//     K: ?Sized + Ord,
//     {
//         a.sort_by(|x, y| key(x).cmp(key(y)));
//     }
// }
// impl<T> Tokenizer<T> for Lexer<T>
// {
//     fn tokenize(&mut self, text : &str, def : Vec<TokenDefinition<T>>)-> Vec<Token<T>>
//     {
//         self.definitions = def;
//         let mut groups : HashMap<usize, Vec<TokenMatch<T>>> = HashMap::new();
//         let matches = self.find_matches(text);
//         let mut position = 0;
//         for m in matches 
//         {
//             //проыеряем есть ли в хеше такой стартовый индекс
//             //если нет то добавляем его и новый вектор
//             //добавляем в вектор значение
//             groups.entry(m.start_index)
//                     .or_insert(vec![])
//                     .push(m);
//         }
//         let mut last_match : Option<TokenMatch<T>> = None;
//         let mut tokens: Vec<Token<T>> = Vec::new();
//         for g in groups.iter().sorted_by_key(|s|s.0)
//         {
//             let best_match = g.1.into_iter().sorted_by_key(|s|s.precedence).nth(0).unwrap().clone();
//             if last_match.is_some() && best_match.start_index < last_match.unwrap().end_index
//             {
//                 continue;
//             }
//             position = position + 1;
//             let token = Token::new(best_match.token_type, best_match.value, best_match.start_index, best_match.end_index, best_match.groups, position,  best_match.converted);

//             self.tokens.push(token);
//             last_match = Some(best_match)
//         }
//         tokens
//         //matches.sort_by(|a, b| b.start_index.cmp(&a.start_index));
        
//     }
// }