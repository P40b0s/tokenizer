use std::{borrow::Cow, rc::Rc};

use crate::{lexer::Lexer, token::Token, token_actions::TokenActions};


#[derive(Clone)]
/// после получения массива токенов нам нужно разделить их на ссылку на токен и на ссылку на массив токенов
/// это необходимо для удобства дальнейшей работы с токенами
/// в дальнейшем эту структуру оборачиваем в `TokenActions` и можем работать с токенами, эта логика уже есть в TokenActions, поэтому непосредственно эту структуру использовать нет необходимости
pub struct TokenModel<T>  where T : PartialEq
{
    pub token : Rc<Token<T>>,
    pub tokens : Rc<Vec<Token<T>>>,
}
impl<T> PartialEq for TokenModel<T> where T: PartialEq 
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.token == other.token
    }
}

impl<T> Eq for TokenModel<T> where T: Eq {}

impl<T> TokenModel<T> where T : PartialEq
{
    pub fn new(lexer: &Lexer<T>) -> Vec<TokenModel<T>>
    {
        let mut tokens :Vec<TokenModel<T>> = Vec::new();
        for lx in *lexer.tokens
        {
            let token = TokenModel {token : Rc::new(lx), tokens : Rc::clone(&lexer.tokens)};
            tokens.push(token);
        }
        tokens
    }
}

impl<T> TokenActions for TokenModel<T> where T : PartialEq 
{
    fn get_value(&self) -> &str
    {
        &self.token.value
    }
    fn get_default_group(&self) -> Option<&str> 
    {
       let def = self.get_group(0)?;
       Some(def)
    }
    fn get_group(&self, group_number: usize) -> Option<&str> 
    {
        let gr = self.token.groups.iter().nth(group_number)?;
        let val = gr.get_value();
        Some(val)
    }
}