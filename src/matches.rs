use regex::Captures;

use crate::token_definition::TokenDefinition;

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
    pub fn get_start_index(&self)-> usize
    {
        self.start_index
    }
    pub fn get_end_index(&self)-> usize
    {
        self.end_index
    }
    pub fn get_lenght(&self)-> usize
    {
        self.lenght
    }
    pub fn get_value(&self)-> &str
    {
        self.value.as_ref()
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

impl<T> TokenMatch<T> where T : Copy
{
    fn new(token_type : T, value : &str, groups: Vec<GroupMatch>, converted: Option<String>, start_index : usize, end_index : usize, precedence : u8) -> TokenMatch<T>
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
    ///Поиск токенов по текущим определениям токенов
    pub fn find(definitions : Vec<TokenDefinition<T>>, input : &str) -> Vec<TokenMatch<T>>
    {
        let mut tokens : Vec<TokenMatch<T>> = Vec::new();
        definitions.into_iter().for_each(|def| 
        {
            let matches = def.get_regex().find_iter(input);
            let all_captures = def.get_regex().captures_iter(input);
            let captures = def.get_regex().captures(input);
            let groups = TokenMatch::get_groups(&def, captures);
            //Из 3 попаданий в группах почему то захватывается только одна группа
            //и получается что в нижнем переборе 3 разных значения но группа только первая доля всех!
            for caps in all_captures
            {
                let zero = caps.get(0).unwrap();
                println!("{}", zero.as_str());
            }
            for m in matches
            {
                //Получаем список захваченныхгрупп у данного текста
                //0 группа всегда вхождение целиком
                //пока думаю...
                let mut converted : Option<String> = None;
                if let Some(conv) = &def.converter 
                {
                    if conv.contains_key(m.as_str())
                    {
                        converted = Some(conv.get(m.as_str()).unwrap().clone());
                    } 
                }
                let token = TokenMatch::new(def.return_token, m.as_str(), groups.to_vec(), converted, m.start(), m.end(), def.precedence);
                tokens.push(token);
            }
        });
        tokens
    }
    ///Если есть именованные группы, добавляем их в группы
    fn get_groups(def : &TokenDefinition<T>, cpt : Option<Captures>) -> Vec<GroupMatch>
    {
        let mut v :Vec<GroupMatch> = Vec::new();
        if cpt.is_some()
        {
            let captures = cpt.unwrap();
            def.get_regex().capture_names().for_each(|n|
            {
                if n.is_some()
                {
                    let name = n.unwrap();
                    let capture_name = captures.name(name);
                    if capture_name.is_some()
                    {
                        let capture_name = capture_name.unwrap();
                        let lenght = capture_name.end() - capture_name.start();
                        let gm = GroupMatch::new(name,
                            capture_name.as_str(),
                            capture_name.start(),
                            capture_name.end(),
                            lenght);
                        v.push(gm);
                    }   
                }
            });
        }
        v
    }
}