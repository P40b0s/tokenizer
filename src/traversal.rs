use crate::{lexer::Lexer, token::Token};

pub struct Traversal<'a, T>
{
    tokens : &'a Vec<Token<T>>,
}
///Методы по обходу абстрактного синтаксическго дерева
impl<'a, T> Traversal<'a, T>
{
    pub fn new(lexer : &'a Lexer<T>) -> Traversal<'a, T>
    {
        Traversal { tokens : &lexer.tokens }
    }
    pub fn next(&self, token : &Token<T>) -> Option<&'a Token<T>>
    {
        let s = self.tokens.into_iter().find(|f| f.position == token.position+1);
        s
    }
    pub fn next_is(&self, token : &Token<T>, next : T) -> Option<&'a Token<T>>
    {
        let s = self.lexer.tokens.into_iter().find(|f| f.position == token.position+1);
        let result = match s 
        {
            Some(x) => Some(&x),
            None => None
        };
        result
    }
}