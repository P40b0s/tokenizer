
mod matches;
mod token_definition;
mod lexer;
mod token;
use matches::GroupMatch;
fn main() 
{
    let gm = GroupMatch::new(
        "G1",
        "имечко фамилия",
        0,
        32,
        22);
    println!("Hello, world!");
}
