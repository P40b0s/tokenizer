use crate::matches::GroupMatch;

#[derive(Clone)]
pub struct Token<T>
{
    //tokens : &'a Vec<Token<'a, T>>,
    pub token_type : T,
    pub value : String,
    pub start_index : usize,
    pub end_index : usize,
    pub lenght : usize,
    pub position : usize,
    pub converted_value : Option<String>,
    pub groups : Vec<GroupMatch>
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