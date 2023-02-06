use std::collections::HashMap;
use std::rc::{Rc, Weak};

extern crate tokenizer;
extern crate tokenizer_derive;

use tokenizer_derive::Tokenizer;
use tokenizer::{Token, TokenActions};
use tokenizer::{ForwardTokenActions, BackwardTokenActions, GlobalActions, TokenDefinition, Tokenizer, Lexer};

#[derive(Copy, Clone, PartialEq, Debug, Tokenizer)]
pub enum TestTokens
{
    #[token(pattern="(?P<gr>123)")]
    OneTwoThree,
    #[token(pattern="321")]
    #[token(converter="*>абырвалг")]
    ThreeTwoOne,
    #[token(pattern="000")]
    #[token(converter="000>ZERO")]
    Zero
}

#[derive(Copy, Clone, PartialEq, Debug, Tokenizer)]
pub enum TT
{
    #[token(pattern="123[[[p][]]321")]
    #[token(precedence="3")]
    #[token(converter="123321>321")]
    One,
    Two
}

#[test]
fn test_macros()
{
    let tt : Option<String> = None;
    // let tttt = tt.unwrap_or_else(|| {
    //     println!("ERROR!");
    //     "AADFFF".to_owned()
    // });
    let tt = TT::get_defs();
    if tt.is_none()
    {
        return;
    }
    //let trtrt =  tt.iter().map(|m|m.unw).collect();
    let text = "Тестовый текст 123321 тестовый текст 321 какой то текст 000";
    let tt = tt.unwrap();
    let actions = Lexer::tokenize(text, tt);
    let trtr = "";
}

#[test]
fn next_skip_one_test() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 000";
    let defs =  TestTokens::get_defs();
    if defs.is_none()
    {
        return;
    }
    let actions = Lexer::tokenize(text, defs.unwrap());
    //let actions = GlobalActions::new(&lexer);
    if let Some(first) = actions.get(TestTokens::OneTwoThree)
    {
        if let Some(next) = first.next_skip(1)
        {
            let token = next.token;
            let skip_one = token.token_type;
            assert_eq!(TestTokens::Zero, skip_one);
        }
    }
}


#[derive(Copy, Clone, PartialEq, Debug, Tokenizer)]
pub enum GroupTestTokens
{
    #[token(pattern="[А-Яа-я0-9_]+=(?P<gr>.*)")]
    KeyValue,
}
#[test]
fn groups_test() 
{
    let text = r#"Тестирование групп
    0=первая группа
    1=вторая группа
    2=третья группа
    все конец"#;
    if let Some(defs) = GroupTestTokens::get_defs()
    {
        let actions = Lexer::tokenize(text, defs);
        //let actions = GlobalActions::new(&lexer);
        if let Some(first) = actions.get(GroupTestTokens::KeyValue)
        {
            assert_eq!(first.get_first_group().unwrap(), "первая группа".to_owned());
            let sec_token = first.next().unwrap();
            assert_eq!(sec_token.get_first_group().unwrap(), "вторая группа".to_owned());
            let thr_token = sec_token.next().unwrap();
            assert_eq!(thr_token.get_first_group().unwrap(), "третья группа".to_owned());
        }
    }
}

#[test]
fn converter_test() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 000";
    let defs =  TestTokens::get_defs();
    if defs.is_none()
    {
        return;
    }
    let actions = Lexer::tokenize(text, defs.unwrap());
   //let actions = GlobalActions::new(&lexer);
    if let Some(first) = actions.get(TestTokens::OneTwoThree)
    {
        if let Some(next) = first.next()
        {
            let token = next.token;
            assert_eq!(String::from("абырвалг"), *token.converted_value.as_ref().unwrap())
        }
        if let Some(next) = first.next_skip(1)
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
    let defs =  TestTokens::get_defs();
    if defs.is_none()
    {
        return;
    }
    let actions = Lexer::tokenize(text, defs.unwrap());
    //let actions = GlobalActions::new(&lexer);
    if let Some(first) = actions.get(TestTokens::Zero)
    {
        if let Some(next) = first.next()
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
    let defs =  TestTokens::get_defs();
    if defs.is_none()
    {
        return;
    }
    let actions = Lexer::tokenize(text, defs.unwrap());
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
    let defs =  TestTokens::get_defs();
    if defs.is_none()
    {
        return;
    }
    let actions = Lexer::tokenize(text, defs.unwrap());
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

