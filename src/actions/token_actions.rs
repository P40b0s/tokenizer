use std::ptr::eq;
use crate::{lexer::Lexer, token_model::TokenModel, token::Token};

///После получения структуры TokenModel создаем на основе нее структуру TokenActions
/// c помощью нее мы будем получать необходимые токены из массива
/// в которой будут располагатся методы для работы с абстрактным синтаксическим деревом
pub struct TokenActions<'a, T> where T :  PartialEq + Clone
{
    pub tokens : Vec<TokenModel<'a, T>>,
}
impl<'a, T> TokenActions<'a, T> where T :  PartialEq + Clone
{
    // pub fn new2(lexer : Lexer<T>) -> TokenActions<'a, T>
    // {
    //     let toks = TokenModel::new(&lexer);
    //     TokenActions { tokens : toks, lexer}
    // }
    pub fn new(lexer : &'a Lexer<T>) -> TokenActions<T>
    {
        let toks = TokenModel::new(lexer);
        TokenActions { tokens : toks}
    }
    ///Получает первый встречающийся токен `token_type` типа
    pub fn get(&self, token_type : T) -> Option<&TokenModel<T>>
    {
        //let searched_token = Token::get_equality_token(token_type);
        let founded = self.tokens.iter().find(|f|f.token.token_type == token_type);
        founded
    }
    ///Возвращает токен на позиции `position`
    pub fn get_by_position(&self, position : usize) -> Option<&TokenModel<T>>
    {
        let founded = self.tokens.iter().find(|f|f.token.position == position);
        founded
    }
}






