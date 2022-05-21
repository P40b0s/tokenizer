
mod matches;
mod token_definition;
mod lexer;
mod token;
mod token_model;
mod token_actions;

use matches::GroupMatch;

use crate::{token_definition::TokenDefinition, matches::TokenMatch, lexer::{Lexer, Tokenizer}, token_actions::TokenActions, token_model::TokenModel};
#[derive(Copy, Clone, PartialEq, Debug)]
enum Test
{
    OneTwoThree,
    ThreeTwoOne,
    Test3
}
fn get_test_definitions() -> Result<Vec<TokenDefinition<Test>>, regex::Error>
{
    let mut definitions : Vec<TokenDefinition<Test>> = Vec::new();
    let td1 = TokenDefinition::new(Test::OneTwoThree, "(?P<gr>123)", 0, None)?;
    let td2 = TokenDefinition::new(Test::ThreeTwoOne, r"321", 0, None)?;
    definitions.push(td1);
    definitions.push(td2);
    Ok(definitions)
}
fn main() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 321";
    let defs : Result<Vec<TokenDefinition<Test>>, regex::Error> = get_test_definitions();
    if defs.is_err()
    {
        println!("Ошибка в регексе: {}", defs.err().unwrap());
        return;
    }
    let lexer = Lexer::tokenize(text, defs.unwrap());
    let traversal = TokenActions::new(&lexer);
    let first = traversal.get_first(Test::OneTwoThree);
    if first.is_some()
    {
        let next = traversal.next(first.unwrap(), 0);
    }

    let gm = GroupMatch::new(
        "G1",
        "имечко фамилия",
        0,
        32,
        22);
    println!("Hello, world!");
}
