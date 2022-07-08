use std::{collections::HashMap};

use crate::matches::{GroupMatch, TokenMatch};
use regex::{Regex, Match, Captures, Error};
impl GroupMatch
{
    pub fn add()
    {
        print!("sdsdsdsd")
    }
}
#[derive(Debug, Clone)]
///Определение токена, определенный регекс с весом по которому будет вестись поиск в тексте
pub struct TokenDefinition<T> where T : Clone 
{
    pub regex : Regex,
    pub return_token : T,
    pub precedence : u8,
    pub converter : Option<HashMap<String,String>>
}
impl<T> TokenDefinition<T> where T : Clone
{
    /// # Arguments
    ///
    /// * `return_token` - Тип токена, для которого будет создано определение
    /// * `regex_pattern` - Регекс для поиска данного токена в тексте
    /// * `precedence` - Вес токена, если регексы перекрывают друг друга, то определением токена станет то у которого самый низкий вес (начинается с  0)
    /// * `converter` - При необходимости, конвертирование значения в другое
    ///
    pub fn new(return_token : T, regex_pattern : &str, precedence : u8, converter : Option<HashMap<String,String>>) -> Result<TokenDefinition<T>, Error>
    {
        let rx = Regex::new(regex_pattern)?;
        Ok(TokenDefinition 
        {
            return_token,
            regex : rx,
            precedence,
            converter
        })
    }
}