use std::rc::Rc;

use crate::{Token, TokenActions, matches::GroupMatch};



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

// impl<T> TokenModel<T> where T : PartialEq + Clone
// {
    
// }

impl<T> TokenActions<T> for TokenModel<T> where T : PartialEq + Clone
{
    fn get_value(&self) -> &str
    {
        &self.token.value
    }
    fn get_first_group(&self) -> Option<&str> 
    {
       let def = self.get_group(1)?;
       Some(def)
    }
    fn get_group(&self, group_number: usize) -> Option<&str> 
    {
        let gr = self.token.groups.iter().find(|f| f.get_group_index() == group_number);
        let val: Option<&str> = gr.map_or(None, |m| Some(m.get_value()));
        val
    }
    fn get_group_by_name(&self, group_name: &str) -> Option<&GroupMatch>
    {
        if let Some(gr) = self.token.groups.iter().find(|f|f.get_name().is_some_and(|n| n == group_name))
        {
            Some(gr)
        }
        else 
        {
            None
        }
    }
    fn get_position(&self) -> usize
    {
        self.token.position
    }
    fn get_tokentype(&self) -> &T
    {
        &self.token.token_type
    }
    fn is_tokentype(&self, token_type: T) -> bool
    {
        &self.token.token_type == &token_type
    }
    fn get_tokens(&self) -> core::slice::Iter<Token<T>>
    {
        self.tokens.iter()
    }
    fn to_token_model(&self, t: &Token<T>) -> TokenModel<T>
    {
        TokenModel { token: t.clone(), tokens: Rc::clone(&self.tokens) }
    }
    fn get_coordinates(&self) -> (usize, usize)
    {
        (self.token.start_index, self.token.lenght)
    }
    fn get_converted_value(&self) -> Option<&String>
    {
        self.token.converted_value.as_ref()
    }

}