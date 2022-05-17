use std::collections::HashMap;

use crate::matches::{GroupMatch, TokenMatch};
use regex::{Regex, Match, Captures};
impl GroupMatch
{
    pub fn add()
    {
        print!("sdsdsdsd")
    }
}
#[derive(Debug, Clone)]
pub struct TokenDefinition<T>
{
    regex : Regex,
    return_token : T,
    precedence : u8,
    converter : Option<HashMap<String,String>>
}

impl<T> TokenDefinition<T>
{
    pub fn new(return_token : T, regex_pattern : &str, precedence : u8, converter : Option<HashMap<String,String>>) -> TokenDefinition<T>
    {
        TokenDefinition 
        {
            return_token,
            regex : Regex::new(regex_pattern).unwrap(),
            precedence,
            converter
        }
    }

    pub fn find_matches(self, input : &str) -> Vec<TokenMatch<T>>
    {
        let matches = self.regex.find_iter(input);
        let captures = self.regex.captures(input);
        let groups = self.get_groups(captures);
        let mut tokens : Vec<TokenMatch<T>> = Vec::new();
        for m in matches
        {
            let mut converted : Option<String> = None;
            if self.converter.is_some() && self.converter.as_ref().unwrap().contains_key(m.as_str())
            {
                converted = Some(self.converter.as_ref().unwrap().get(m.as_str()).unwrap().clone());
            }
            let def = self.clone();
            let token = TokenMatch::new(def.return_token, m.as_str(), groups.to_vec(), converted, m.start(), m.end(), def.precedence);
            tokens.push(token);
        }
        tokens
    }
    //Получаем группы если они есть
    fn get_groups(&self, cpt : Option<Captures>) -> Vec<GroupMatch>
    {
        let mut v :Vec<GroupMatch> = Vec::new();
        if cpt.is_some()
        {
            let captures = cpt.unwrap();
            for (pos, c) in captures.iter().enumerate()
            {
                if c.is_some()
                {
                    let curr_group = c.unwrap();
                    //получаем имя группы равное позиции группы
                    let c_name = self.regex.capture_names().nth(pos);
                    let mut name = "";
                    if c_name.is_some()
                    {
                        if c_name.unwrap().is_some()
                        {
                            name = c_name.unwrap().unwrap();
                        }
                    }
                    let lenght = curr_group.end() - curr_group.start();
                    let gm = GroupMatch::new(name, curr_group.as_str(), curr_group.start(), curr_group.end(), lenght);
                    v.push(gm);
                }   
            }
        }
        v
    }
}