use crate::{token_model::TokenModel, global_actions::TokenActions, token::Token};


pub(crate) trait BackwardTokenActions<'a, T> where T :  PartialEq + Clone
{
    fn before(&self, skip : usize) -> Option<TokenModel<T>>;
    fn before_is(&self, next : T, skip : usize) -> bool;
    ///Ищем токены переданные в фунции predicate вниз по массиву с максимальной глубиной max_deep и включая себя with_self
    fn find_backward_many(&self, searched_tokens : &dyn Fn(&Token<T>) -> bool, max_deep : usize, with_self : bool) -> Option<TokenModel<T>>;
    ///Ищем один из заданных токенов, игнорируем заданные токены, если встречается токен отличный от игнорируемых то функия возвратит None
    fn find_backward_many_ignore(&self, searched_tokens : &dyn Fn(&Token<T>) -> bool, ignored_tokens : &dyn Fn(&Token<T>) -> bool, with_self : bool) -> Vec<TokenModel<T>>;
    ///Поиск токенов вниз по массиву, вернется любой найденный токен кроме указанных в функции `ignore_tokens`
    fn find_backward_ignore(&self, ignore_tokens : &dyn Fn(&Token<T>) -> bool) -> Option<TokenModel<T>>;
    ///ищет указанный токен с максимальной глубиной поиска max_deep
    fn find_backward(&self, searched_token : T, max_deep : usize) -> Option<TokenModel<T>>;
}

impl<'a, T> BackwardTokenActions<'a, T> for TokenModel<'a, T> where T :  PartialEq + Clone
{
    
    ///в моей C# версии парсера было 2 метода - от позиции и от старт индекса, оставлю от старт индекса это точно уникальное число
    fn before(&self, skip : usize) -> Option<TokenModel<T>>
    {
        let start = self.
                                            tokens.
                                            iter().
                                            find(|f|f.end_index == self.token.end_index);
        if  start.is_some()
        {
            let founded = self.
                                                tokens.
                                                iter().
                                                find(|f|f.position == start.unwrap().position -1 - skip);
            return Some(TokenModel { token : founded.unwrap(), tokens : self.tokens});
        }
        None
    }
    fn before_is(&self, before : T, skip : usize) -> bool
    {
        let n = self.before(skip);
        if n.is_some() && n.unwrap().token.token_type == before
        {
            return true;
        }
        false
    }
    ///Ищем токены переданные в фунции predicate вниз по массиву с максимальной глубиной max_deep и включая себя with_self
    fn find_backward_many(&self,
         searched_tokens : &dyn Fn(&Token<T>) -> bool,
         max_deep : usize,
         with_self : bool) -> Option<TokenModel<T>>
    {
        let mut start_position = self.token.position;
        if !with_self
        {
            start_position = self.token.position - 1;
        }
        let mut deep = 0;
        //клонируем, так как нам не нужен перевернутый массив в оригинале
        let mut tmp_tokens = self.tokens.clone();
        tmp_tokens.reverse();
        for t in &tmp_tokens
        {
            if t.position <= start_position
            {
                if searched_tokens(t)
                {
                    //если находим нужный токет, то ищем его в основном массиве и возвращаем ссылку
                    if let Some(t) = self.
                    tokens.
                    iter().
                    find(|f|f.end_index == t.end_index)
                    {
                        return Some(TokenModel { token : t, tokens : self.tokens});
                    }
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
    fn find_backward_many_ignore(&self,
        searched_tokens : &dyn Fn(&Token<T>) -> bool,
        ignored_tokens : &dyn Fn(&Token<T>) -> bool,
        with_self : bool) -> Vec<TokenModel<T>>
    {
        let mut start_position = self.token.position;
        let mut tokens : Vec<TokenModel<T>> = Vec::new();
        if !with_self
        {
            start_position = self.token.position - 1;
        }
        let mut tmp_tokens = self.tokens.clone();
        tmp_tokens.reverse();
        for t in &tmp_tokens
        {
            if t.position <= start_position && (searched_tokens(t) || ignored_tokens(t))
            {
                if searched_tokens(t)
                {
                    if let Some(t) = self.
                    tokens.
                    iter().
                    find(|f|f.end_index == t.end_index)
                    {
                        tokens.push(TokenModel { token : t, tokens : self.tokens});
                    }
                    
                }
            }
        }
        tokens
    }
    ///Поиск токенов вниз по массиву, вернется любой найденный токен кроме указанных в функции `ignore_tokens`
    fn find_backward_ignore(&self,
        ignore_tokens : &dyn Fn(&Token<T>) -> bool) -> Option<TokenModel<T>>
    {
        let mut tmp_tokens = self.tokens.clone();
        tmp_tokens.reverse();
        for t in &tmp_tokens
        {
            if t.start_index <= self.token.start_index
            {
                if !ignore_tokens(t)
                {
                    if let Some(t) = self.
                    tokens.
                    iter().
                    find(|f|f.end_index == t.end_index)
                    {
                        return Some(TokenModel { token : t, tokens : self.tokens});
                    } 
                }
            }
        }
        None
    }
    ///ищет указанный токен с максимальной глубиной поиска max_deep
    fn find_backward(&self,
        searched_token : T,
        max_deep : usize) -> Option<TokenModel<T>>
    {
        let sr = self.find_backward_many(&|f| f.token_type == searched_token, max_deep, false);
        sr
    }
}