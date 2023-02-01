use std::rc::Rc;

use crate::{token_model::TokenModel, token::Token, TokenActions};


pub trait BackwardTokenActions<T> where T :  PartialEq + Clone
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

impl<T> BackwardTokenActions<T> for TokenModel<T> where T :  PartialEq + Clone
{
    
    ///в моей C# версии парсера было 2 метода - от позиции и от старт индекса, оставлю от старт индекса это точно уникальное число
    fn before(&self, skip : usize) -> Option<TokenModel<T>>
    {
        let token = self.get_tokens()
                                    .find(|f|f.position == (self.get_position() - 1 - skip))?;

        let model = self.to_token_model(token);
        Some(model)
    }
    fn before_is(&self, before : T, skip : usize) -> bool
    {
        if let Some(n) = self.before(skip)
        {
            if n.token.eq_type(&before)
            {
                return true;
            }
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
        let original = Rc::as_ref(&self.tokens);
        if original.len() > 0
        {
            let len = original.len() -1;
            for i in (0usize.. len).rev()
            {
                let val = original.iter().nth(i)?;
                if val.start_index <= start_position
                {
                    if searched_tokens(val)
                    {
                        return Some(self.to_token_model(val));
                    }
                    deep = deep + 1;
                    if deep == max_deep
                    {
                        break;
                    }
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
        let original = Rc::as_ref(&self.tokens);
        if original.len() > 0
        {
            let len = original.len() -1;
            for i in (0usize.. len).rev()
            {
                if let Some(val) = original.iter().nth(i)
                {
                    if val.start_index <= start_position
                    {
                        if searched_tokens(val)
                        {
                            tokens.push(self.to_token_model(val));
                        }
                        if ignored_tokens(val)
                        {
                            continue;
                        }
                        else
                        {
                            break;
                        }
                    }
                }
            }
        }
        // let mut tmp_tokens = *link.clone();
        // tmp_tokens.reverse();
        // for t in &tmp_tokens
        // {
        //     if t.token.position <= start_position
        //     {
        //         if searched_tokens(&t.token)
        //         {
        //             tokens.push(t);
        //         }
        //         if ignored_tokens(&t.token)
        //         {
        //             continue;
        //         }
        //         else
        //         {
        //             break;
        //         }
        tokens
    }
    ///Поиск токенов вниз по массиву, вернется любой найденный токен кроме указанных в функции `ignore_tokens`
    fn find_backward_ignore(&self,
        ignore_tokens : &dyn Fn(&Token<T>) -> bool) -> Option<TokenModel<T>>
    {
        let original = Rc::as_ref(&self.tokens);
        if original.len() > 0
        {
            let len = original.len() -1;
            for i in (0usize.. len).rev()
            {
                let val = original.iter().nth(i)?;
                if val.start_index <= self.token.start_index
                {
                    if !ignore_tokens(val)
                    {
                        return Some(self.to_token_model(val));
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