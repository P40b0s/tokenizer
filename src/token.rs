use crate::{matches::GroupMatch};

#[derive(Clone)]
pub struct Token<T> where T: PartialEq
{
    pub token_type : T,
    pub value : String,
    pub start_index : usize,
    pub end_index : usize,
    pub lenght : usize,
    pub position : usize,
    pub converted_value : Option<String>,
    pub groups : Vec<GroupMatch>
}

impl<T> PartialEq for Token<T> where T: PartialEq 
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.token_type == other.token_type
    }
}

impl<T> Eq for Token<T> where T: Eq {}

impl<T> Token<T> where T :  PartialEq
{
    ///Получение пустой структуры для сравнения (почему то напрямую T мы сравнивать не можем)
    // pub fn get_equality_token(token_type : T) -> Token<T>
    // {
    //     Token
    //     {
    //         token_type,
    //         value : String::from(""),
    //         start_index : 0,
    //         end_index : 0,
    //         groups : Vec::new(),
    //         position : 0,
    //         converted_value : None,
    //         lenght : 0
    //     }
    // }
    pub fn new(ttype : T,
        value : String,
        start_index : usize,
        end_index : usize,
        groups : Vec<GroupMatch>,
        position : usize,
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
            converted_value : converted,
            lenght : end_index - start_index
        }
    }
}