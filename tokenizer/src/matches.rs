use std::collections::HashMap;

use regex::Captures;

use crate::token_definition::TokenDefinition;

#[derive(Debug, Clone)]
pub struct GroupMatch
{
    value : String,
    start_index : usize,
    end_index : usize,
    lenght : usize,
    name: Option<String>
}

impl GroupMatch
{
    pub fn new(value : &str, start_index : usize, end_index : usize, lenght : usize, name: Option<String>) -> GroupMatch
    {
        GroupMatch 
        {
            value : value.to_owned(),
            start_index,
            end_index,
            lenght,
            name
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
    pub fn get_name(&self)-> Option<&String>
    {
        self.name.as_ref()
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
    //Имя группы и ее  найденое значение
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
            // let mut names : HashMap<String, String> = HashMap::new();
            // if let Some(cps) = def.get_regex().captures(input)
            // {
            //     names = def.get_regex()
            //     .capture_names()
            //     .flatten()
            //     .filter_map(|n| Some((n.to_owned(), cps.name(n)?.as_str().to_owned())))
            //     .collect();
            // }
            
            //let matches = def.get_regex().find_iter(input);
            let all_captures = def.get_regex().captures_iter(input);
            
            //let captures = def.get_regex().captures(input);
            //let groups = TokenMatch::get_groups(&def, captures);
            //Из 3 попаданий в группах почему то захватывается только одна группа
            //и получается что в нижнем переборе 3 разных значения но группа только первая для всех!
            for caps in all_captures
            {
                let all_match = caps.get(0).expect("Ошибка извлечения 0 группы из вхождения(такого не может быть)");
                let mut groups: Vec<GroupMatch>  = vec![];
                for i in 1..10
                {   
                    if let Some(gr) = caps.get(i)
                    {
                        let mut name: Option<String> = None;
                        for n in def.get_regex().capture_names()
                        {
                            if let Some(n) = n
                            {
                                let nm = caps.name(n).map_or("", |m| m.as_str());
                                if caps.name(n).is_some() && nm == gr.as_str()
                                {
                                    name = Some(n.to_owned())
                                }
                            }
                        }
                        let start = gr.start();
                        let end = gr.end();
                        let lenght = end - start;
                        let gm = GroupMatch::new
                        (
                            gr.as_str(),
                            start,
                            end,
                            lenght,
                            name
                        );
                        groups.push(gm);
                    }
                    else
                    {
                        //Если нет хотя бы первой группы то дальше их быть не может, итд.
                        break;
                    }
                }   
                let mut converted : Option<String> = None;
                if let Some(conv) = &def.converter 
                {
                    if conv.contains_key("*")
                    {
                        converted = Some(conv.get("*").unwrap().clone());
                    }
                    else
                    if conv.contains_key(all_match.as_str())
                    {
                        converted = Some(conv.get(all_match.as_str()).unwrap().to_owned());
                    } 
                    // else
                    // {
                    //     for i in 0..10
                    //     {   
                    //         let key = ["gr", i.to_string().as_str()].concat();
                    //         if conv.contains_key(&key)
                    //         {
                    //             converted = Some(conv.get(&key).unwrap().to_owned());
                    //         }
                    //     }
                    // }
                }
                let token = TokenMatch::new(def.return_token, all_match.as_str(), groups,  converted, all_match.start(), all_match.end(), def.precedence);
                tokens.push(token);
            }
            // for m in matches
            // {
            //     //Получаем список захваченныхгрупп у данного текста
            //     //0 группа всегда вхождение целиком
            //     //пока думаю...
            //     let mut converted : Option<String> = None;
            //     if let Some(conv) = &def.converter 
            //     {
            //         if conv.contains_key(m.as_str())
            //         {
            //             converted = Some(conv.get(m.as_str()).unwrap().clone());
            //         } 
            //     }
            //     let token = TokenMatch::new(def.return_token, m.as_str(), groups.to_vec(), converted, m.start(), m.end(), def.precedence);
            //     tokens.push(token);
            // }
        });
        tokens
    }
}
    //Если есть именованные группы, добавляем их в группы
    // fn get_groups(def : &TokenDefinition<T>, cpt : Option<Captures>) -> Vec<GroupMatch>
    // {
    //     let mut v :Vec<GroupMatch> = Vec::new();
    //     if cpt.is_some()
    //     {
    //         let captures = cpt.unwrap();
    //         def.get_regex().capture_names().for_each(|n|
    //         {
    //             if n.is_some()
    //             {
    //                 let name = n.unwrap();
    //                 let capture_name = captures.name(name);
    //                 if capture_name.is_some()
    //                 {
    //                     let capture_name = capture_name.unwrap();
    //                     let lenght = capture_name.end() - capture_name.start();
    //                     let gm = GroupMatch::new(name,
    //                         capture_name.as_str(),
    //                         capture_name.start(),
    //                         capture_name.end(),
    //                         lenght);
    //                     v.push(gm);
    //                 }   
    //             }
    //         });
    //     }
    //     v
    // }
