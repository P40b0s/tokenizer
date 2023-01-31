use std::{collections::HashMap, sync::Mutex, rc::Rc, cell::RefCell};

use crate::matches::{GroupMatch};
use regex::{Regex, Error};
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
    pub fn new(return_token : T, regex_pattern : &str, precedence : u8, converter : Option<[&str; 2]>) -> Result<TokenDefinition<T>, Error>
    {
        let regex = Regex::new(regex_pattern)?;

        Ok(TokenDefinition 
        {
            return_token,
            regex,
            precedence,
            converter : match converter
            {
                Some(c) => 
                {
                    let mut converter: HashMap<String, String> = HashMap::new();
                    converter.insert(String::from(c[0]), String::from(c[1]));
                    Some(converter)
                },
                None => None
            }
        })
    }

    pub fn get_regex(&self)-> &Regex
    {
        &self.regex
    }
}

pub struct TokenDefinitionsBuilder<T>(Vec<TokenDefinition<T>>) where T: Clone;
impl<T> TokenDefinitionsBuilder<T> where T: Clone
{
    pub fn new() -> Self
    {
        let  v : Vec<TokenDefinition<T>> = vec![];
        let builder = TokenDefinitionsBuilder(v);
        builder
    }
    ///Добавить связки токенов и др.
    pub fn add_def(&mut self, return_token : T, regex_pattern : &str) -> Result<&mut Self, Error>
    {
        let token = TokenDefinition::new(return_token, regex_pattern, 0,None)?;
        self.0.push(token);
        Ok(self)
    }
     ///Добавить связки токенов и др.
     pub fn add_custom_def(&mut self, return_token : T, regex_pattern : &str, precedence : u8, converter : Option<[&str; 2]>) -> Result<&mut Self, Error>
     {
         let token = TokenDefinition::new(return_token, regex_pattern, precedence,converter)?;
         self.0.push(token);
         Ok(self)
     }
    ///Вернуть массив определений токенов
    pub fn build(&mut self) -> Vec<TokenDefinition<T>>
    {
        self.0.clone()
    }
}


