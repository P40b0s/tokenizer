use std::collections::HashMap;

use regex::Captures;

use crate::token_definition::TokenDefinition;

#[derive(Debug, Clone)]
pub struct GroupMatch
{
    ///индекс группы в найденом выражении
    index: usize,
    value : String,
    start_index : usize,
    end_index : usize,
    lenght : usize,
    name: Option<String>
}

impl GroupMatch
{
    pub fn new(index: usize, value : &str, start_index : usize, end_index : usize, lenght : usize, name: Option<String>) -> GroupMatch
    {
        GroupMatch 
        {
            index,
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
    pub fn get_group_index(&self)-> usize
    {
        self.index
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
            for caps in def.get_regex().captures_iter(input)
            {
                let mut groups: Vec<GroupMatch>  = vec![];
                if let Some(all_match) = caps.get(0)
                { 
                    for (i, c) in caps.iter().enumerate()
                    {
                        if let Some(gr) = c
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
                                i,
                                gr.as_str(),
                                start,
                                end,
                                lenght,
                                name
                            );
                            groups.push(gm);
                        }
                    }
                    // for i in 1..10
                    // {   
                    //     if let Some(gr) = caps.get(i)
                    //     {
                    //         let mut name: Option<String> = None;
                    //         for n in def.get_regex().capture_names()
                    //         {
                    //             if let Some(n) = n
                    //             {
                    //                 let nm = caps.name(n).map_or("", |m| m.as_str());
                    //                 if caps.name(n).is_some() && nm == gr.as_str()
                    //                 {
                    //                     name = Some(n.to_owned())
                    //                 }
                    //             }
                    //         }
                    //         let start = gr.start();
                    //         let end = gr.end();
                    //         let lenght = end - start;
                    //         let gm = GroupMatch::new
                    //         (
                    //             gr.as_str(),
                    //             start,
                    //             end,
                    //             lenght,
                    //             name
                    //         );
                    //         groups.push(gm);
                    //     }
                        //группы с ленивым квантификатором будут None, но за ними может быть еще нумерованная группа
                        //возьмем максимум 10 групп
                        // else
                        // {
                        //     //Если нет хотя бы первой группы то дальше их быть не может, итд.
                        //     break;
                        // }
                    // }   
                    let mut converted : Option<String> = None;
                    if let Some(conv) = &def.converter 
                    {

                        let mut cv = String::new();
                        caps.expand(conv, &mut cv);
                        converted = Some(cv);
                    }
                    let token = TokenMatch::new(def.return_token, all_match.as_str(), groups.clone(),  converted, all_match.start(), all_match.end(), def.precedence);
                    tokens.push(token);
                }
            }
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
