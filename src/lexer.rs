

use std::{collections::HashMap};
use crate::{token_definition::TokenDefinition, matches::TokenMatch, token::Token};
use itertools::Itertools;

pub(crate) trait Tokenizer<T> where T : PartialEq + Clone
{
    /// Токинезация текста
    /// ### Arguments
    ///
    /// * `text` - Текст для токинезации
    /// * `defs` - Список определений токенов с помощью которых будет проводится поиск
    ///
    /// ### Examples
    ///
    /// ```
    /// //Делаем список определений токенов
    /// let mut defs : Vec<TokenDefinition<TestTokens>> = Vec::new();
    /// let td1 = TokenDefinition::new(TestTokens::OneTwoThree, "(?P<gr>123)", 0, None)?;
    /// let td2 = TokenDefinition::new(TestTokens::ThreeTwoOne, r"321", 0, None)?;
    /// defs.push(td1);
    /// defs.push(td2);
    /// //сам текст
    /// let text = "Тестовый текст 123 тестовый текст 321 какой то текст 321";
    /// //Токинезируем текст
    /// let lexer = Lexer::tokenize(text, defs);
    /// //оборачиваем для дальнейшей работы
    /// let traversal = TokenActions::new(&lexer);
    /// ```
    fn tokenize(text : &str, defs : Vec<TokenDefinition<T>>)-> Lexer<T>;
}
/// В начале нужно запустить лексер, он найдет все токены с заданными `TokenDefinition`
/// Затем оборачиваем лексер в TokenActions и можем работать с токенами
pub struct Lexer<T> where T : PartialEq
{
    pub tokens : Vec<Token<T>>,
}

impl<T> Tokenizer<T> for Lexer<T> where T : Copy + Clone + PartialEq
{
    ///Поиск токенов по текущему тексту и заданным определениям токенов
    fn tokenize(text : &str, defs : Vec<TokenDefinition<T>>)-> Lexer<T>
    {
        let tokens_match = TokenMatch::find(defs, text);
        let mut groups : HashMap<usize, Vec<TokenMatch<T>>> = HashMap::new();
        let mut position = 0;
        for m in tokens_match
        {
            //проыеряем есть ли в хеше такой стартовый индекс
            //если нет то добавляем его и новый вектор
            //добавляем в вектор значение
            groups.entry(m.start_index)
                    .or_insert(vec![])
                    .push(m);
        }
        let mut last_match : Option<TokenMatch<T>> = None;
        let mut tokens: Vec<Token<T>> = Vec::new();
        for g in groups.iter().sorted_by_key(|s|s.0)
        {
            let best_match = g.1.into_iter().sorted_by_key(|s|s.precedence).nth(0).unwrap().clone();
            if last_match.is_some()
            {
                let last_index = last_match.as_ref().unwrap().end_index;
                if best_match.start_index < last_index
                {
                    continue;
                }
            }
            position = position + 1;
            last_match = Some(best_match.clone());
            let token = Token::new(best_match.token_type,
                best_match.value,
                best_match.start_index,
                best_match.end_index,
                best_match.groups,
                position,
                best_match.converted);
            tokens.push(token);
        }
        Lexer{ tokens }
    }
}