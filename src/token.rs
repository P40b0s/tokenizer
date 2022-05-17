use crate::matches::GroupMatch;

#[derive(Clone)]
pub struct Token<T>
{
    //tokens : &'a Vec<Token<'a, T>>,
    token_type : T,
    value : String,
    start_index : usize,
    end_index : usize,
    lenght : usize,
    position : usize,
    converted_value : Option<String>,
    groups : Vec<GroupMatch>
}

impl<T> Token<T>
{
    pub fn new(ttype : T,
        value : String,
        start_index : usize,
        end_index : usize,
        groups : Vec<GroupMatch>,
        position : usize,
        //tokens: &'a Vec<Token<'a, T>>,
        converted : Option<String>) -> Token<T>
    {
        Token
        {
            token_type : ttype,
            value,
            start_index,
            end_index,
            groups,
            position,
            //tokens,
            converted_value : converted,
            lenght : end_index - start_index

        }
    }
}