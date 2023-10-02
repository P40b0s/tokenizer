use crate::TokenModel;

///После получения структуры TokenModel создаем на основе нее структуру TokenActions
///Глобально получает необходимый токен из массива
pub struct GlobalActions<T> where T :  PartialEq + Clone
{
    pub tokens : Vec<TokenModel<T>>,
}
impl<T> GlobalActions<T> where T :  PartialEq + Clone
{
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






