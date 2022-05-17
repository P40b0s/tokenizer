

#[derive(Debug, Clone)]
pub struct GroupMatch
{
    pub name : String,
    value : String,
    start_index : usize,
    end_index : usize,
    lenght : usize
}

impl GroupMatch
{
    pub fn new(name : &str, value : &str, start_index : usize, end_index : usize, lenght : usize) -> GroupMatch
    {
        GroupMatch 
        {
            name : name.to_owned(),
            value : value.to_owned(),
            start_index,
            end_index,
            lenght
        }
    }
}
#[derive(Debug, Clone)]
pub struct TokenMatch<T>
{
    /// Тип токена
    pub token_type : T,
    /// Значение токена
    pub value : String,
    /// Массив групп если они были определены в определениях токенов
    pub groups : Vec<GroupMatch>,
    /// Конвертированое значение (если был задан конвертер в определениях токенов)
    pub converted : Option<String>,
    /// Начальный индекс токена в текущей строке
    pub start_index : usize,
    /// Конечный индекс токена в текущей строке
    pub end_index : usize,
    /// Приоритет При нахождении токенов с одинаковым вхождением приоритет будет отдаваться тому токену у которого меньше значение
    pub precedence : u8
}

impl<T> TokenMatch<T>
{
    pub fn new(token_type : T, value : &str, groups: Vec<GroupMatch>, converted: Option<String>, start_index : usize, end_index : usize, precedence : u8) -> TokenMatch<T>
    {
        TokenMatch
        {
            token_type,
            value : value.to_owned(),
            groups,
            converted,
            start_index,
            end_index,
            precedence
        }
    }
}