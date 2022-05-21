use std::ptr::eq;
use crate::{lexer::Lexer, token_model::TokenModel, token::Token};

///После получения структуры TokenModel создаем на основе нее структуру TokenActions
/// c помощью нее мы будем получать необходимые токены из массива
/// в которой будут располагатся методы для работы с абстрактным синтаксическим деревом
#[derive(Clone)]
pub struct TokenActions<'a, T> where T :  PartialEq
{
    tokens : Vec<TokenModel<'a, T>>
}
impl<'a, T> TokenActions<'a, T> where T :  PartialEq
{
    pub fn new(lexer : &'a Lexer<T>) -> TokenActions<T>
    {
        let toks = TokenModel::new(lexer);
        TokenActions { tokens : toks}
    }
    pub fn get(&self, token_type : T) -> Option<&TokenModel<T>>
    {
        //let searched_token = Token::get_equality_token(token_type);
        let founded = self.tokens.iter().find(|f|f.token.token_type == token_type);
        founded
    }
    pub fn get_by_position(&self, position : usize) -> Option<&TokenModel<T>>
    {
        let founded = self.tokens.iter().find(|f|f.token.position == position);
        founded
    }
}

pub(crate) trait ForwardTokenActions<'a, T> where T :  PartialEq + Clone
{
    fn next(&self, token: &TokenModel<T>, skip : usize) -> Option<&TokenModel<T>>;
    fn next_is(&self, token: &TokenModel<T>, next : T, skip : usize) -> bool;
    ///Ищем токены переданные в фунции predicate вниз по массиву с максимальной глубиной max_deep и включая себя with_self
    fn find_forward_many(&self, token: &TokenModel<T>, searched_tokens : &dyn Fn(&TokenModel<T>) -> bool, max_deep : usize, with_self : bool) -> Option<&TokenModel<T>>;
    ///Ищем один из заданных токенов, игнорируем заданные токены, если встречается токен отличный от игнорируемых то функия возвратит None
    fn find_forward_many_ignore(&self, token: &TokenModel<T>, searched_tokens : &dyn Fn(&TokenModel<T>) -> bool, ignored_tokens : &dyn Fn(&TokenModel<T>) -> bool, with_self : bool) -> Vec<&TokenModel<T>>;
    ///Поиск токенов вниз по массиву, вернется любой найденный токен кроме указанных в функции `ignore_tokens`
    fn find_forward_ignore(&self, token: &TokenModel<T>, ignore_tokens : &dyn Fn(&TokenModel<T>) -> bool) -> Option<&TokenModel<T>>;
    ///ищет указанный токен с максимальной глубиной поиска max_deep
    fn find_forward(&self, token: &TokenModel<T>, searched_token : &TokenModel<T>, max_deep : usize) -> Option<&TokenModel<T>>;
}

impl<'a, T> ForwardTokenActions<'a, T> for TokenActions<'a, T> where T :  PartialEq + Clone
{
    
    ///в моей C# версии парсера было 2 метода - от позиции и от старт индекса, оставлю от старт индекса это точно уникальное число
    fn next(&self, token: &TokenModel<T>, skip : usize) -> Option<&TokenModel<T>>
    {
        let start = self.
                                            tokens.
                                            iter().
                                            find(|f|f.token.start_index == token.token.start_index);
        if  start.is_some()
        {
            let founded = self.
                                                tokens.
                                                iter().
                                                find(|f|f.token.position == start.unwrap().token.position +1 + skip);
            return founded;
        }
        None
    }
    fn next_is(&self, token: &TokenModel<T>, next : T, skip : usize) -> bool
    {
        let n = self.next(token, skip);
        if n.is_some() && n.unwrap().token.token_type == next
        {
            return true;
        }
        false
    }
    ///Ищем токены переданные в фунции predicate вниз по массиву с максимальной глубиной max_deep и включая себя with_self
    fn find_forward_many(&self, token: &TokenModel<T>, searched_tokens : &dyn Fn(&TokenModel<T>) -> bool, max_deep : usize, with_self : bool) -> Option<&TokenModel<T>>
    {
        let mut start_position = token.token.position;
        if !with_self
        {
            start_position = token.token.position +1;
        }
        let mut deep = 0;
        for t in &self.tokens
        {
            if t.token.position >= start_position
            {
                if searched_tokens(t)
                {
                    return Some(t);
                }
                deep = deep + 1;
                if deep == max_deep
                {
                    break;
                }
            }
        }
        None
    }
    ///Ищем один из заданных токенов, игнорируем заданные токены, если встречается токен отличный от игнорируемых то функия возвратит None
    fn find_forward_many_ignore(&self, token: &TokenModel<T>, searched_tokens : &dyn Fn(&TokenModel<T>) -> bool, ignored_tokens : &dyn Fn(&TokenModel<T>) -> bool, with_self : bool) -> Vec<&TokenModel<T>>
    {
        let mut start_position = token.token.position;
        let mut tokens : Vec<&TokenModel<T>> = Vec::new();
        if !with_self
        {
            start_position = token.token.position +1;
        }
        for t in &self.tokens
        {
            if t.token.position >= start_position && (searched_tokens(t) || ignored_tokens(t))
            {
                if searched_tokens(t)
                {
                    tokens.push(t);
                }
            }
        }
        tokens
    }
    ///Поиск токенов вниз по массиву, вернется любой найденный токен кроме указанных в функции `ignore_tokens`
    fn find_forward_ignore(&self, token: &TokenModel<T>, ignore_tokens : &dyn Fn(&TokenModel<T>) -> bool) -> Option<&TokenModel<T>>
    {
        for t in &self.tokens
        {
            if t.token.start_index >= token.token.start_index
            {
                if !ignore_tokens(t)
                {
                    return Some(t);
                }
            }
        }
        None
    }
    ///ищет указанный токен с максимальной глубиной поиска max_deep
    fn find_forward(&self, token: &TokenModel<T>, searched_token : &TokenModel<T>, max_deep : usize) -> Option<&TokenModel<T>>
    {
        let sr = self.find_forward_many(token, &|f| f == searched_token, max_deep, false);
        sr
    }
}

pub(crate) trait BackwardTokenActions<'a, T> where T :  PartialEq + Clone
{
    fn before(&self, token: &TokenModel<T>, skip : usize) -> Option<&TokenModel<T>>;
    fn before_is(&self, token: &TokenModel<T>, next : T, skip : usize) -> bool;
    ///Ищем токены переданные в фунции predicate вниз по массиву с максимальной глубиной max_deep и включая себя with_self
    fn find_backward_many(&self, token: &TokenModel<T>, searched_tokens : &dyn Fn(&TokenModel<T>) -> bool, max_deep : usize, with_self : bool) -> Option<&TokenModel<T>>;
    ///Ищем один из заданных токенов, игнорируем заданные токены, если встречается токен отличный от игнорируемых то функия возвратит None
    fn find_backward_many_ignore(&self, token: &TokenModel<T>, searched_tokens : &dyn Fn(&TokenModel<T>) -> bool, ignored_tokens : &dyn Fn(&TokenModel<T>) -> bool, with_self : bool) -> Vec<&TokenModel<T>>;
    ///Поиск токенов вниз по массиву, вернется любой найденный токен кроме указанных в функции `ignore_tokens`
    fn find_backward_ignore(&self, token: &TokenModel<T>, ignore_tokens : &dyn Fn(&TokenModel<T>) -> bool) -> Option<&TokenModel<T>>;
    ///ищет указанный токен с максимальной глубиной поиска max_deep
    fn find_backward(&self, token: &TokenModel<T>, searched_token : &TokenModel<T>, max_deep : usize) -> Option<&TokenModel<T>>;
}

impl<'a, T> BackwardTokenActions<'a, T> for TokenActions<'a, T> where T :  PartialEq + Clone
{
    
    ///в моей C# версии парсера было 2 метода - от позиции и от старт индекса, оставлю от старт индекса это точно уникальное число
    fn before(&self, token: &TokenModel<T>, skip : usize) -> Option<&TokenModel<T>>
    {
        let start = self.
                                            tokens.
                                            iter().
                                            find(|f|f.token.end_index == token.token.end_index);
        if  start.is_some()
        {
            let founded = self.
                                                tokens.
                                                iter().
                                                find(|f|f.token.position == start.unwrap().token.position -1 - skip);
            return founded;
        }
        None
    }
    fn before_is(&self, token: &TokenModel<T>, before : T, skip : usize) -> bool
    {
        let n = self.before(token, skip);
        if n.is_some() && n.unwrap().token.token_type == before
        {
            return true;
        }
        false
    }
    ///Ищем токены переданные в фунции predicate вниз по массиву с максимальной глубиной max_deep и включая себя with_self
    fn find_backward_many(&self, token: &TokenModel<T>, searched_tokens : &dyn Fn(&TokenModel<T>) -> bool, max_deep : usize, with_self : bool) -> Option<&TokenModel<T>>
    {
        let mut start_position = token.token.position;
        if !with_self
        {
            start_position = token.token.position - 1;
        }
        let mut deep = 0;
        let mut tmp_tokens = self.tokens.clone();
        tmp_tokens.reverse();
        for t in &tmp_tokens
        {
            if t.token.position <= start_position
            {
                if searched_tokens(t)
                {
                    return Some(t);
                }
                deep = deep + 1;
                if deep == max_deep
                {
                    break;
                }
            }
        }
        None
    }
    ///Ищем один из заданных токенов, игнорируем заданные токены, если встречается токен отличный от игнорируемых то функия возвратит None
    fn find_forward_many_ignore(&self, token: &TokenModel<T>, searched_tokens : &dyn Fn(&TokenModel<T>) -> bool, ignored_tokens : &dyn Fn(&TokenModel<T>) -> bool, with_self : bool) -> Vec<&TokenModel<T>>
    {
        let mut start_position = token.token.position;
        let mut tokens : Vec<&TokenModel<T>> = Vec::new();
        if !with_self
        {
            start_position = token.token.position +1;
        }
        for t in &self.tokens
        {
            if t.token.position >= start_position && (searched_tokens(t) || ignored_tokens(t))
            {
                if searched_tokens(t)
                {
                    tokens.push(t);
                }
            }
        }
        tokens
    }
    ///Поиск токенов вниз по массиву, вернется любой найденный токен кроме указанных в функции `ignore_tokens`
    fn find_forward_ignore(&self, token: &TokenModel<T>, ignore_tokens : &dyn Fn(&TokenModel<T>) -> bool) -> Option<&TokenModel<T>>
    {
        for t in &self.tokens
        {
            if t.token.start_index >= token.token.start_index
            {
                if !ignore_tokens(t)
                {
                    return Some(t);
                }
            }
        }
        None
    }
    ///ищет указанный токен с максимальной глубиной поиска max_deep
    fn find_forward(&self, token: &TokenModel<T>, searched_token : &TokenModel<T>, max_deep : usize) -> Option<&TokenModel<T>>
    {
        let sr = self.find_forward_many(token, &|f| f == searched_token, max_deep, false);
        sr
    }
}
