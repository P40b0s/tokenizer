use std::{borrow::Cow, rc::Rc};



pub trait TokenActions
{
    ///Возвращает значение токена
    fn get_value(&self) -> &str;
    fn get_default_group(&self) -> Option<&str>;
    fn get_group(&self, group_number: usize) -> Option<&str>;
}