
mod matches;
mod token_definition;
mod lexer;
mod token;
use std::process::exit;

use matches::GroupMatch;

use crate::{token_definition::TokenDefinition, matches::TokenMatch};
#[derive(Copy, Clone)]
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
    let text = "Тестовый текст 123 тестовый текст 321";
    let defs : Result<Vec<TokenDefinition<Test>>, regex::Error> = get_test_definitions();
    if defs.is_err()
    {
        println!("Ошибка в регексе: {}", defs.err().unwrap());
        return;
    }
    let tokens_match = TokenMatch::find(defs.unwrap(), text);

    let gm = GroupMatch::new(
        "G1",
        "имечко фамилия",
        0,
        32,
        22);
    println!("Hello, world!");
}
