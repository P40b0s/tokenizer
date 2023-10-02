use crate::{Token, TokenModel};

pub trait TokenActions<T> where T: PartialEq + Clone
{
    ///Возвращает значение токена
    fn get_value(&self) -> &str;
    fn get_first_group(&self) -> Option<&str>;
    fn get_group(&self, group_number: usize) -> Option<&str>;
    fn get_position(&self) -> usize;
    fn get_tokentype(&self) -> &T;
    fn get_tokens(&self)-> core::slice::Iter<Token<T>>;
    fn to_token_model(&self, t: &Token<T>) -> TokenModel<T>;
}