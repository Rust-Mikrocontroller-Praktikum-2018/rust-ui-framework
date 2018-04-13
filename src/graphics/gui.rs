use graphics::{ui, ui_component::UIComponent};
use alloc::{Vec, String, boxed::Box, string::ToString};
use stm32f7::lcd::Color;

pub struct Model {
    pub screen: usize,
}

pub fn generate_model() -> Model {
    Model{
        screen: 0,
    }
}

#[derive(Clone)]
pub enum Message{
}

pub fn view(m: &Model) -> Vec<Box<UIComponent>> {
    vec![]
}

pub fn update(m: Model, msg: Message) -> Model{
    m
}