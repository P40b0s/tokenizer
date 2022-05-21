use crate::token_definition::TokenDefinition;
use crate::lexer::{Tokenizer, Lexer};
use crate::token_actions::{TokenActions,ForwardTokenActions, BackwardTokenActions};


#[derive(Copy, Clone, PartialEq, Debug)]
enum TestTokens
{
    OneTwoThree,
    ThreeTwoOne,
    Test3
}
fn get_test_definitions() -> Result<Vec<TokenDefinition<TestTokens>>, regex::Error>
{
    let mut definitions : Vec<TokenDefinition<TestTokens>> = Vec::new();
    let td1 = TokenDefinition::new(TestTokens::OneTwoThree, "(?P<gr>123)", 0, None)?;
    let td2 = TokenDefinition::new(TestTokens::ThreeTwoOne, r"321", 0, None)?;
    definitions.push(td1);
    definitions.push(td2);
    Ok(definitions)
}

#[test]
fn it_adds_two() 
{
    let text = "Тестовый текст 123 тестовый текст 321 какой то текст 321";
    let defs : Result<Vec<TokenDefinition<TestTokens>>, regex::Error> = get_test_definitions();
    if defs.is_err()
    {
        println!("Ошибка в регексе: {}", defs.err().unwrap());
        return;
    }
    let lexer = Lexer::tokenize(text, defs.unwrap());
    let traversal = TokenActions::new(&lexer);
    let first = traversal.get(TestTokens::OneTwoThree);
    if first.is_some()
    {
        let next = traversal.next(first.unwrap(), 1);
        let skip_one = next.unwrap().token.token_type;
        assert_eq!(TestTokens::ThreeTwoOne, skip_one);
    }
    
}
