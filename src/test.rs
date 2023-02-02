use std::collections::HashMap;
use std::rc::{Rc, Weak};

use tokenizer_derive::Tokenizer;

use crate::Token;
use crate::forward_actions::ForwardTokenActions;
use crate::backward_actions::BackwardTokenActions;
use crate::token_definition::{TokenDefinition, Definitions, TokenDefinitionsBuilder};
use crate::lexer::{Tokenizer, Lexer};
use crate::global_actions::{GlobalActions};


pub trait CreateDefinitions where Self: Clone
{
    fn create_defs(&self) -> Result<Vec<TokenDefinition<Self>>, regex::Error>;
}


#[derive(Copy, Clone, PartialEq, Debug, Tokenizer)]
enum TestTokens
{
    #[token(pattern="123321")]
    #[token(precedence="3")]
    #[token(converter="123>321")]
    OneTwoThree,
    #[token(pattern="0000")]
    ThreeTwoOne,
    #[token(pattern="hsdjfhwuiegf")]
    Zero
}

#[test]
fn test_macros()
{
    let tt : Vec<TokenDefinition<TestTokens>> = TokenDefinition::get_defs();
    let trtr = "";
}


fn get_test_definitions() -> Result<Vec<TokenDefinition<TestTokens>>, regex::Error>
{
    let mut builder = TokenDefinitionsBuilder::<TestTokens>::new();
    let defs = builder
    .add_custom_def(TestTokens::OneTwoThree, "(?P<gr>123)", 0, None)?
    .add_custom_def(TestTokens::ThreeTwoOne, r"321", 0, None)?
    .add_custom_def(TestTokens::Zero, r"000", 0, Some(["000", "ZERO"]))?.build();
    Ok(defs)
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
    let actions = Lexer::tokenize(text, get_definitions().unwrap());
    //let actions = GlobalActions::new(&lexer);
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
fn groups_test() 
{
    let text = r#"Тестирование групп
    0=первая группа
    1=вторая группа
    2=третья группа
    все конец"#;
    let actions = Lexer::tokenize(text, get_definitions().unwrap());
    //let actions = GlobalActions::new(&lexer);
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
    let actions = Lexer::tokenize(text, get_definitions().unwrap());
   //let actions = GlobalActions::new(&lexer);
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
    let actions = Lexer::tokenize(text, get_definitions().unwrap());
    //let actions = GlobalActions::new(&lexer);
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
    let actions = Lexer::tokenize(text, get_definitions().unwrap());
    //let actions = GlobalActions::new(&lexer);
    if let Some(first) = actions.get(TestTokens::OneTwoThree)
    {
        if let Some(next) = first.find_forward(&[TestTokens::Zero], 2, false)
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
    let actions = Lexer::tokenize(text, get_definitions().unwrap());
    //let actions = GlobalActions::new(&lexer);
    if let Some(first) = actions.get(TestTokens::Zero)
    {
        if let Some(next) = first.find_backward(TestTokens::OneTwoThree, 2)
        {
            let skip_one = next.token.token_type;
            assert_eq!(TestTokens::OneTwoThree, skip_one);
        } 
    }
}

