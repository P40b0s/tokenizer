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
    #[token(pattern("(?P<gr>123)"))]
    OneTwoThree,
    #[token(pattern("321"), converter("*>абырвалг"))]
    ThreeTwoOne,
    #[token(pattern("000"), converter("000>ZERO"))]
    Zero
}

#[derive(Copy, Clone, PartialEq, Debug, Tokenizer)]
pub enum TestTokens2
{
    #[token(precedence(0), converter("conv1>o"), pattern("pat1"))]
    #[token(precedence(1), converter("conv2>1"), pattern("pat2"))]
    OneTwoThree
}

#[derive(Copy, Clone, PartialEq, Debug, Tokenizer)]
pub enum TT
{
    #[token(pattern("123[[[p][]]321"), precedence(3), converter("123321>321"))]
    One,
    Two
}
///FIXME если добавить #[doc=""] или так ///... то будет ошибка cannot infer type, надо с этим что то сделать
/// похоже что попадет токен документации, надо его как то отсеять
#[derive(Debug, Copy, PartialEq, Clone, Tokenizer)]
enum LtrTokens
{
    #[token(pattern(r#"\[[ПИСЬМО]{6}.*\]"#))]
    Root,
    #[token(pattern(r#"(?i)тема=([^\n\r]+)"#))]
    Theme,
    #[token(pattern(r#"(?i)автоотправка=([^\n\r]+)"#))]
    IsAutosend,
    #[token(pattern(r#"(?i)эцп=([^\n\r]+)"#))]
    IsEds,
    #[token(pattern(r#"(?i)доставлено=([^\n\r]+)"#))]
    IsDelivered,
    #[token(pattern(r#"(?i)прочтено=([^\n\r]+)"#))]
    IsReading,
    #[token(pattern(r#"(?i)дата=([^\n\r]+)"#))]
    Date,
    #[token(pattern(r#"(?i)\[АДРЕСАТЫ\]"#))]
    Addressees,
    #[token(pattern(r#"\[ФАЙЛЫ\]"#))]
    Files,
    #[token(pattern(r#"\[ПИСЬМО.*\]"#))]
    File,
    #[token(pattern(r#"\[ТЕКСТ\]"#))]
    Text,
    #[token(pattern(r#"\d=([^\n\r]+)"#))]
    NumberKey
}

#[test]
fn test_new_macros()
{
    let tt : Option<String> = None;
    // let tttt = tt.unwrap_or_else(|| {
    //     println!("ERROR!");
    //     "AADFFF".to_owned()
    // });
    let tt = TestTokens2::get_defs();
    println!("{:?}", tt);
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
fn test_tets_tokens()
{
    let tt : Option<String> = None;
    // let tttt = tt.unwrap_or_else(|| {
    //     println!("ERROR!");
    //     "AADFFF".to_owned()
    // });
    let tt = LtrTokens::get_defs();
    println!("{:?}", tt);
    if tt.is_none()
    {
        return;
    }
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
    #[token(pattern("[А-Яа-я0-9_]+=(?P<gr>.*)"), precedence(3))]
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

#[derive(Copy, Clone, PartialEq, Debug, Tokenizer)]
pub enum GroupNameTestTokens
{
    #[token(pattern(r#"(?P<one>первый)\s(?P<two>второй)\s(?P<three>третий)"#), converter(r#"конвертированное значение бдет=$three"#),)]
    KeyValue,
}
#[test]
fn groups_names_test() 
{
    let text = r#"первый второй третий"#;
    if let Some(defs) = GroupNameTestTokens::get_defs()
    {
        let actions = Lexer::tokenize(text, defs);
        //let actions = GlobalActions::new(&lexer);
        if let Some(first) = actions.get(GroupNameTestTokens::KeyValue)
        {
            assert_eq!(first.get_group_by_name("one").unwrap().get_value(), "первый");
            assert_eq!(first.get_group_by_name("two").unwrap().get_value(), "второй");
            assert_eq!(first.get_group_by_name("three").unwrap().get_value(), "третий");
            assert_eq!(first.token.converted_value.as_ref().unwrap(), "конвертированное значение бдет=третий");

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

