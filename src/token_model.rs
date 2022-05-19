use crate::{lexer::Lexer, token::Token};


#[derive(Clone)]
/// после получения массива токенов нам нужно разделить их на ссылку на токен и на ссылку на массив токенов
/// это необходимо для удобства дальнейшей работы с токенами
/// так же эта структура будет использоваться для поиска по массиву от текущего токена
pub struct TokenModel<'a, T>  where T : PartialEq
{
    pub token : &'a Token<T>,
    pub tokens : &'a Vec<Token<T>>,
}
impl<'a, T> PartialEq for TokenModel<'a, T> where T: PartialEq 
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.token == other.token
    }
}

impl<'a, T> Eq for TokenModel<'a, T> where T: Eq {}
impl<'a, T> TokenModel<'a, T> where T : PartialEq
{
    pub fn new(lexer: &'a Lexer<T>) -> Vec<TokenModel<'a, T>>
    {
        let mut tokens : Vec<TokenModel<'a, T>> = Vec::new();
        for lx in &lexer.tokens
        {
            let token = TokenModel {token : lx, tokens : &lexer.tokens};
            tokens.push(token);
        }
        tokens
    }
}


