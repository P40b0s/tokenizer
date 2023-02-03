use std::rc::Rc;
use crate::{TokenModel, Token, TokenActions};


pub trait ForwardTokenActions<T> where T :  PartialEq + Clone
{
    fn next(&self) -> Option<TokenModel<T>>;
    fn next_skip(&self, skip : usize) -> Option<TokenModel<T>>;
    fn next_is(&self, next : T, skip : usize) -> bool;
    ///Ищем токены переданные в фунции predicate вниз по массиву с максимальной глубиной max_deep и включая себя with_self
    fn find_forward(&self, searched_tokens :&[T], max_deep : usize, with_self : bool) -> Option<TokenModel<T>>;
    ///Ищем один из заданных токенов, игнорируем заданные токены, если встречается токен отличный от игнорируемых или искомых, функция остановится
    fn find_forward_many_ignore(&self, searched_tokens : &dyn Fn(&Token<T>) -> bool, ignored_tokens : &dyn Fn(&Token<T>) -> bool, with_self : bool) -> Vec<TokenModel<T>>;
    ///Поиск токенов вниз по массиву, вернется любой найденный токен кроме указанных в функции `ignore_tokens`
    fn find_forward_first_ignore(&self, ignore_tokens : &dyn Fn(&Token<T>) -> bool) -> Option<TokenModel<T>>;
    ///Берет указанные токены пока не встретиться отличный от них
    fn take_forward_while_predicate(&self, predicate : &dyn Fn(&Token<T>) -> bool) -> Vec<TokenModel<T>>;
    fn take_forward_while(&self,tokens : &[T]) -> Vec<TokenModel<T>>;
}

impl<T> ForwardTokenActions<T> for TokenModel<T> where T :  PartialEq + Clone
{
    
    ///Получает следующий по массиву токен, если skip = 0
    fn next(&self) -> Option<TokenModel<T>>
    {
        self.next_skip(0)
    }
     ///Получает следующий по массиву токен, если skip = 0
     fn next_skip(&self, skip : usize) -> Option<TokenModel<T>>
     {
         let founded = self.get_tokens()
                                 .find(|f|f.position == (self.get_position() +1 + skip))?;
         let model = self.to_token_model(founded);
         Some(model)
     }

    fn next_is(&self, next : T, skip : usize) -> bool
    {
        if let Some(n) = self.next_skip(skip)
        {
            if n.token.eq_type(&next)
            {
                return true;
            }
        }
        false
    }
    ///Ищем токены переданные в фунции predicate вниз по массиву с максимальной глубиной max_deep и включая себя with_self
    fn find_forward(&self,
         searched_tokens :&[T],
         max_deep : usize,
         with_self : bool) -> Option<TokenModel<T>>
    {
        let mut start_position = self.token.position;
        if !with_self
        {
            start_position = self.token.position +1;
        }
        let mut deep = 0;
        let len = self.tokens.len() -1;
        for i in 0usize.. len
        {
            let val = self.get_tokens().nth(i)?;
            if val.position >= start_position
            {
                if searched_tokens.contains(&val.token_type)
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
        None
    }
    ///Ищем один из заданных токенов, игнорируем заданные токены, если встречается токен отличный от игнорируемых или заданных то поиск закончится
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
        for t in Rc::as_ref(&self.tokens)
        {
            if t.position >= start_position
            {
                if searched_tokens(&t)
                {
                    tokens.push(self.to_token_model(t));
                }
                if ignored_tokens(&t)
                {
                    continue;
                }
                else
                {
                    break;
                }
            }
        }
        tokens
    }
    ///Поиск токенов вниз по массиву, вернется первый найденный токен кроме указанных в функции `ignore_tokens`
    fn find_forward_first_ignore(&self,
        ignore_tokens : &dyn Fn(&Token<T>) -> bool) -> Option<TokenModel<T>>
    {
        for t in Rc::as_ref(&self.tokens)
        {
            if t.start_index >= self.token.start_index
            {
                if !ignore_tokens(&t)
                {
                    return Some(self.to_token_model(t));
                }
            }
        }
        None
    }
    ///ищет указанный токен с максимальной глубиной поиска max_deep
    // fn find_forward(&self,
    //     searched_token : T,
    //     max_deep : usize) -> Option<TokenModel<T>>
    // {
    //     let sr = self.find_forward_many(&|f| f.eq_type(&searched_token), max_deep, false);
    //     sr
    // }


    ///Ищем один из заданных токенов, игнорируем заданные токены, если встречается токен отличный от игнорируемых или заданных то поиск закончится
    fn take_forward_while_predicate(&self,
        predicate : &dyn Fn(&Token<T>) -> bool) -> Vec<TokenModel<T>>
    {
        let start_position = self.token.position +1;
        let mut tokens : Vec<TokenModel<T>> = Vec::new();
        for t in Rc::as_ref(&self.tokens)
        {
            if t.position >= start_position
            {
                if !predicate(&t)
                {
                    break;
                }
                else
                {
                    tokens.push(self.to_token_model(t));
                }
            }
        }
        tokens
    }

    fn take_forward_while(&self,searched : &[T]) -> Vec<TokenModel<T>>
    {
        let start_position = self.token.position +1;
        let mut tokens : Vec<TokenModel<T>> = Vec::new();
        for t in Rc::as_ref(&self.tokens)
        {
            if t.position >= start_position
            {
                if !searched.contains(&t.token_type)
                {
                    break;
                }
                else
                {
                    tokens.push(self.to_token_model(t));
                }
            }
        }
        tokens
    }
}