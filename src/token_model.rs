use std::rc::Rc;

use crate::{token::Token, token_actions::TokenActions};


#[derive(Clone)]
/// после получения массива токенов нам нужно разделить их на ссылку на токен и на ссылку на массив токенов
/// это необходимо для удобства дальнейшей работы с токенами
/// в дальнейшем эту структуру оборачиваем в `TokenActions` и можем работать с токенами, эта логика уже есть в TokenActions, поэтому непосредственно эту структуру использовать нет необходимости
pub struct TokenModel<T>  where T : PartialEq + Clone
{
    pub token : Token<T>,
    pub tokens : Rc<Vec<Token<T>>>,
    //pub myself: Option<Weak<Vec<TokenModel<T>>>>
}
impl<T> PartialEq for TokenModel<T> where T: PartialEq + Clone
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.token == other.token
    }
}

impl<T> Eq for TokenModel<T> where T: Eq + Clone{}

impl<T> TokenModel<T> where T : PartialEq + Clone
{
    // pub fn new(lexer: &Lexer<T>) -> Vec<TokenModel<T>>
    // {
    //     let mut tokens :Vec<TokenModel<T>> = Vec::new();
    //     for lx in *lexer.tokens
    //     {
    //         let token = TokenModel {token : Rc::new(lx), tokens : Rc::clone(&lexer.tokens), myself: None};
    //         tokens.push(token);
    //     }
    //     tokens
    // }

    // pub fn add_ref(&mut self, myself: Weak<Vec<TokenModel<T>>>)
    // {
    //     self.myself = Some(myself);
    // }
}

impl<T> TokenActions<T> for TokenModel<T> where T : PartialEq + Clone
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
    fn get_position(&self) -> usize
    {
        self.token.position
    }
    fn get_tokentype(&self) -> &T
    {
        &self.token.token_type
    }
    fn get_tokens(&self) -> core::slice::Iter<Token<T>>
    {
        self.tokens.iter()
    }
    fn to_token_model(&self, t: &Token<T>) -> TokenModel<T>
    {
        TokenModel { token: t.clone(), tokens: Rc::clone(&self.tokens) }
    }
}