

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
    ///use tokenizer::forward_actions::ForwardTokenActions;
    ///use tokenizer::backward_actions::BackwardTokenActions;
    ///use tokenizer::token_definition::TokenDefinition;
    ///use tokenizer::lexer::{Tokenizer, Lexer};
    ///use tokenizer::global_actions::{TokenActions};
    /// enum TestTokens
    ///{
    ///    OneTwoThree,
    ///    ThreeTwoOne,
    ///    Zero
    ///}
    /// //Создается список определений токенов
    /// let mut defs : Vec<TokenDefinition<TestTokens>> = Vec::new();
    /// let td1 = TokenDefinition::new(TestTokens::OneTwoThree, "(?P<gr>123)", 0, None)?;
    /// let td2 = TokenDefinition::new(TestTokens::ThreeTwoOne, r"321", 0, None)?;
    /// defs.push(td1);
    /// defs.push(td2);
    /// //сам текст
    /// let text = "Тестовый текст 123 тестовый текст 321 какой то текст 321";
    /// //Токинезирует текст
    /// let lexer = Lexer::tokenize(text, defs);
    /// //оборачивает для дальнейшей работы
    /// let traversal = TokenActions::new(&lexer);
    /// //Получает первый токен, от которого будет осуществлять поиск
    /// if let Some(first) = actions.get(TestTokens::OneTwoThree)
    ///{
    /// //В дальнейшем для поиска используются трейты BackwardTokenActions и ForwardTokenActions
    ///    if let Some(next) = first.next(1)
    ///    {
    ///        let token = next.token;
    ///        let skip_one = token.token_type;
    ///        assert_eq!(TestTokens::Zero, skip_one);
    ///    }
    ///}
    /// ```
    fn tokenize(text : &str, defs : Vec<TokenDefinition<T>>)->  Lexer<T>;
}
/// В начале нужно запустить лексер, он найдет все токены с заданными `TokenDefinition`
/// Затем оборачиваем лексер в TokenActions и можем работать с токенами
pub struct Lexer<T> where T : PartialEq
{
    pub tokens : Vec<Token<T>>
}

impl<T> Tokenizer<T> for Lexer<T> where T : Copy + Clone + PartialEq
{
    ///Поиск токенов по текущему тексту и заданным определениям токенов
    fn tokenize(text : &str, defs : Vec<TokenDefinition<T>>)-> Self
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