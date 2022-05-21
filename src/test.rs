use std::collections::HashMap;

use crate::token_definition::TokenDefinition;
use crate::lexer::{Tokenizer, Lexer};
use crate::token_actions::{TokenActions,ForwardTokenActions, BackwardTokenActions};
use crate::token_model::TokenModel;


#[derive(Copy, Clone, PartialEq, Debug)]
enum TestTokens
{
    OneTwoThree,
    ThreeTwoOne,
    Zero
}
fn get_test_definitions() -> Result<Vec<TokenDefinition<TestTokens>>, regex::Error>
{
    let mut definitions : Vec<TokenDefinition<TestTokens>> = Vec::new();
    let td1 = TokenDefinition::new(TestTokens::OneTwoThree, "(?P<gr>123)", 0, None)?;
    let td2 = TokenDefinition::new(TestTokens::ThreeTwoOne, r"321", 0, None)?;
    let mut converter: HashMap<String, String> = HashMap::new();
    converter.insert(String::from("000"), String::from("ZERO"));
    let td3 = TokenDefinition::new(TestTokens::Zero, r"000", 0, Some(converter))?;
    definitions.push(td1);
    definitions.push(td2);
    definitions.push(td3);
    Ok(definitions)
}
fn get_definitions() -> Option<Vec<TokenDefinition<TestTokens>>>
{
    let defs : Result<Vec<TokenDefinition<TestTokens>>, regex::Error> = get_test_definitions();
    if defs.is_err()
    {
        println!("Ошибка в регексе: {}", defs.err().unwrap());
        return None;
    }
    Some(defs.unwrap())
}

#[test]
fn next_skip_one_test() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 000";
    let lexer = Lexer::tokenize(text, get_definitions().unwrap());
    let actions = TokenActions::new(&lexer);
    let first = actions.get(TestTokens::OneTwoThree);
    if first.is_some()
    {
        let next = actions.next(first.unwrap(), 1);
        let token = next.unwrap().token;
        let skip_one = token.token_type;
        assert_eq!(TestTokens::Zero, skip_one);
    }
}

#[test]
fn converter_test() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 000";
    let lexer = Lexer::tokenize(text, get_definitions().unwrap());
    let actions = TokenActions::new(&lexer);
    let first = actions.get(TestTokens::OneTwoThree);
    if first.is_some()
    {
        let next = actions.next(first.unwrap(), 1);
        let token = next.unwrap().token;
        assert_eq!(String::from("ZERO"), *token.converted_value.as_ref().unwrap())
    }
}
#[test]
fn before_skip_one_test() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 000";
    let lexer = Lexer::tokenize(text, get_definitions().unwrap());
    let actions = TokenActions::new(&lexer);
    let first = actions.get(TestTokens::Zero);
    if first.is_some()
    {
        let next = actions.before(first.unwrap(), 1);
        let token = next.unwrap().token;
        let skip_one = token.token_type;
        assert_eq!(TestTokens::OneTwoThree, skip_one);
    }
}
#[test]
fn find_forward_test() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 000";
    let lexer = Lexer::tokenize(text, get_definitions().unwrap());
    let actions = TokenActions::new(&lexer);
    let first = actions.get(TestTokens::OneTwoThree);
    if first.is_some()
    {
        let next = actions.find_forward(first.unwrap(), TestTokens::Zero, 2);
        let token = next.unwrap().token;
        let skip_one = token.token_type;
        assert_eq!(TestTokens::Zero, skip_one);
    }
    
}
#[test]
fn find_backward_test() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 000";
    let lexer = Lexer::tokenize(text, get_definitions().unwrap());
    let actions = TokenActions::new(&lexer);
    let first = actions.get(TestTokens::Zero);
    if first.is_some()
    {
        let next = actions.find_backward(first.unwrap(), TestTokens::OneTwoThree, 2);
        let token = next.unwrap().token;
        let skip_one = token.token_type;
        assert_eq!(TestTokens::OneTwoThree, skip_one);
    }
    
}

pub struct  TokenizerModel<T> where T : Clone + PartialEq + Copy
{
    pub text : String,
    pub lexer : Lexer<T>,
}
impl<'a, T> TokenizerModel<T> where T : Clone + PartialEq + Copy
{
    
    pub fn new(text : &str, defs : Vec<Result<TokenDefinition<T>, regex::Error>>) -> Option<TokenizerModel<T>>
    {
        if text.len() == 0
        {
            println!("Текст для токинезации не содержит символов");
            return None;
        }
        let mut local_defs : Vec<TokenDefinition<T>>  = Vec::new();
        for d in defs
        {
            if d.is_err()
            {
                println!("Ошибка в регексе: {}", d.err().unwrap());
                return None;
            }
            else 
            {
                local_defs.push(d.unwrap());
            }
        }
        let lexer = Lexer::tokenize(&text, local_defs);
        let t : TokenizerModel<T> = TokenizerModel { text: text.to_owned(), lexer: lexer};
        Some(t)
        
    }
    pub fn tokenize(&'a self) -> Option<TokenActions<'a, T>>
    {
        Some(TokenActions::new(&self.lexer))
    }
}
