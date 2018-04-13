use graphics::{ui, ui_component::UIComponent};
use alloc::{Vec, String, boxed::Box, string::ToString};
use stm32f7::lcd::Color;

pub struct Model {
    pub screen: Screen,
}

pub fn generate_model() -> Model {
    Model{
        screen: Screen::Main,
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Screen{
    Main,
}

#[derive(Clone)]
pub enum Message{
}

pub fn view(m: &Model) -> Vec<Box<UIComponent>> {
    vec![
        ui::button(20, 20, 100, 30, "Inc".to_string(), Color::rgb(100, 100, 100), None),
        ui::button(20, 60, 100, 30, "Dec".to_string(), Color::rgb(100, 100, 100), None),
        ui::text_element(20, 100, "0".to_string()),
    ]
}

pub fn update(m: Model, msg: Message) -> Model{
    m
}