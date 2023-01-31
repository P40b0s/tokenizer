use std::collections::HashMap;

use crate::forward_actions::ForwardTokenActions;
use crate::backward_actions::BackwardTokenActions;
use crate::token_definition::{TokenDefinition, AddToken};
use crate::lexer::{Tokenizer, Lexer};
use crate::global_actions::{TokenActions};


pub trait CreateDefinitions where Self: Clone
{
    fn create_defs(&self) -> Result<Vec<TokenDefinition<Self>>, regex::Error>;
}


#[derive(Copy, Clone, PartialEq, Debug)]
enum TestTokens
{
    OneTwoThree,
    ThreeTwoOne,
    Zero
}





fn get_test_definitions() -> Result<Vec<TokenDefinition<TestTokens>>, regex::Error>
{
    let tt = TestTokens::OneTwoThree as u32;
    let mut definitions : Vec<TokenDefinition<TestTokens>> = Vec::new();
    definitions.add_token(TestTokens::OneTwoThree, "(?P<gr>123)", 0, None)?;
    definitions.add_token(TestTokens::ThreeTwoOne, r"321", 0, None)?;
    definitions.add_token(TestTokens::Zero, r"000", 0, Some(["000", "ZERO"]))?;
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
    if let Some(first) = actions.get(TestTokens::OneTwoThree)
    {
        if let Some(next) = first.next(1)
        {
            let token = next.token;
            let skip_one = token.token_type;
            assert_eq!(TestTokens::Zero, skip_one);
        }
    }
}

#[test]
fn converter_test() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 000";
    let lexer = Lexer::tokenize(text, get_definitions().unwrap());
    let actions = TokenActions::new(&lexer);
    if let Some(first) = actions.get(TestTokens::OneTwoThree)
    {
        if let Some(next) = first.next(1)
        {
            let token = next.token;
            assert_eq!(String::from("ZERO"), *token.converted_value.as_ref().unwrap())
        }
    }
}
#[test]
fn before_skip_one_test() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 000";
    let lexer = Lexer::tokenize(text, get_definitions().unwrap());
    let actions = TokenActions::new(&lexer);
    if let Some(first) = actions.get(TestTokens::Zero)
    {
        if let Some(next) = first.next(0)
        {
            let skip_one = next.token.token_type;
            assert_eq!(TestTokens::OneTwoThree, skip_one);
        } 
    }
}
#[test]
fn find_forward_test() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 000";
    let lexer = Lexer::tokenize(text, get_definitions().unwrap());
    let actions = TokenActions::new(&lexer);
    if let Some(first) = actions.get(TestTokens::OneTwoThree)
    {
        if let Some(next) = first.find_forward(TestTokens::Zero, 2)
        {
            let skip_one = next.token.token_type;
            assert_eq!(TestTokens::Zero, skip_one);
        }
    }
}
#[test]
fn find_backward_test() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 000";
    let lexer = Lexer::tokenize(text, get_definitions().unwrap());
    let actions = TokenActions::new(&lexer);
    if let Some(first) = actions.get(TestTokens::Zero)
    {
        if let Some(next) = first.find_backward(TestTokens::OneTwoThree, 2)
        {
            let skip_one = next.token.token_type;
            assert_eq!(TestTokens::OneTwoThree, skip_one);
        } 
    }
}