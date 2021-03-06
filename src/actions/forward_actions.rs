use crate::{token_model::TokenModel, token::Token};



pub(crate) trait ForwardTokenActions<'a, T> where T :  PartialEq + Clone
{
    fn next(&self, skip : usize) -> Option<TokenModel<T>>;
    fn next_is(&self, next : T, skip : usize) -> bool;
    ///Ищем токены переданные в фунции predicate вниз по массиву с максимальной глубиной max_deep и включая себя with_self
    fn find_forward_many(&self, searched_tokens : &dyn Fn(&Token<T>) -> bool, max_deep : usize, with_self : bool) -> Option<TokenModel<T>>;
    ///Ищем один из заданных токенов, игнорируем заданные токены, если встречается токен отличный от игнорируемых то функия возвратит None
    fn find_forward_many_ignore(&self, searched_tokens : &dyn Fn(&Token<T>) -> bool, ignored_tokens : &dyn Fn(&Token<T>) -> bool, with_self : bool) -> Vec<TokenModel<T>>;
    ///Поиск токенов вниз по массиву, вернется любой найденный токен кроме указанных в функции `ignore_tokens`
    fn find_forward_ignore(&self, ignore_tokens : &dyn Fn(&Token<T>) -> bool) -> Option<TokenModel<T>>;
    ///ищет указанный токен с максимальной глубиной поиска max_deep
    fn find_forward(&self, searched_token : T, max_deep : usize) -> Option<TokenModel<T>>;
}

impl<'a, T> ForwardTokenActions<'a, T> for TokenModel<'a, T> where T :  PartialEq + Clone
{
    
    ///Получает следующий по массиву токен, если skip = 0
    fn next(&self, skip : usize) -> Option<TokenModel<T>>
    {
        let start = self.
                                            tokens.
                                            iter().
                                            find(|f|f.start_index == self.token.start_index);
        if  start.is_some()
        {
            let founded = self.
                                                tokens.
                                                iter().
                                                find(|f|f.position == start.unwrap().position +1 + skip);
            if founded.is_some()
            {
                return Some(TokenModel { token : founded.unwrap(), tokens : self.tokens});
            }
            
        }
        None
    }
    fn next_is(&self, next : T, skip : usize) -> bool
    {
        let n = self.next(skip);
        if n.is_some() && n.unwrap().token.token_type == next
        {
            return true;
        }
        false
    }
    ///Ищем токены переданные в фунции predicate вниз по массиву с максимальной глубиной max_deep и включая себя with_self
    fn find_forward_many(&self,
         searched_tokens : &dyn Fn(&Token<T>) -> bool,
         max_deep : usize,
         with_self : bool) -> Option<TokenModel<T>>
    {
        let mut start_position = self.token.position;
        if !with_self
        {
            start_position = self.token.position +1;
        }
        let mut deep = 0;
        for t in self.tokens
        {
            if t.position >= start_position
            {
                if searched_tokens(t)
                {
                    return Some(TokenModel { token : t, tokens : self.tokens});
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
    fn find_forward_many_ignore(&self,
         searched_tokens : &dyn Fn(&Token<T>) -> bool,
         ignored_tokens : &dyn Fn(&Token<T>) -> bool,
         with_self : bool) -> Vec<TokenModel<T>>
    {
        let mut start_position = self.token.position;
        let mut tokens : Vec<TokenModel<T>> = Vec::new();
        if !with_self
        {
            start_position = self.token.position +1;
        }
        for t in self.tokens
        {
            if t.position >= start_position && (searched_tokens(t) || ignored_tokens(t))
            {
                if searched_tokens(t)
                {
                    tokens.push(TokenModel { token : t, tokens : self.tokens});
                }
            }
        }
        tokens
    }
    ///Поиск токенов вниз по массиву, вернется любой найденный токен кроме указанных в функции `ignore_tokens`
    fn find_forward_ignore(&self,
        ignore_tokens : &dyn Fn(&Token<T>) -> bool) -> Option<TokenModel<T>>
    {
        for t in self.tokens
        {
            if t.start_index >= self.token.start_index
            {
                if !ignore_tokens(t)
                {
                    return Some(TokenModel { token : t, tokens : self.tokens});
                }
            }
        }
        None
    }
    ///ищет указанный токен с максимальной глубиной поиска max_deep
    fn find_forward(&self,
        searched_token : T,
        max_deep : usize) -> Option<TokenModel<T>>
    {
        let sr = self.find_forward_many(&|f| f.token_type == searched_token, max_deep, false);
        sr
    }
}