use crate::{Token, TokenModel};

pub trait TokenActions<T> where T: PartialEq + Clone
{
    ///Возвращает значение токена
    fn get_value(&self) -> &str;
    fn get_first_group(&self) -> Option<&str>;
    //Получение значение по номеру группы (ничиная с нуля)
    fn get_group(&self, group_number: usize) -> Option<&str>;
    fn get_group_by_name(&self, group_name: &str) -> Option<&crate::matches::GroupMatch>;
    fn get_position(&self) -> usize;
    fn get_tokentype(&self) -> &T;
    fn is_tokentype(&self, token_type: T) -> bool;
    fn get_tokens(&self)-> core::slice::Iter<Token<T>>;
    fn to_token_model(&self, t: &Token<T>) -> TokenModel<T>;
    //отдает индекс начала вхождения и длинну вхождения в искомой строке
    fn get_coordinates(&self) -> (usize, usize);
}