use graphics::{ui, ui_component::UIComponent};
use alloc::{Vec, String, boxed::Box, string::ToString};
use stm32f7::lcd::Color;

pub struct Model {
    pub screen: Screen,
    counter: i32,
}

pub fn generate_model() -> Model {
    Model{
        screen: Screen::Main,
        counter: 0,
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Screen{
    Main,
}

#[derive(Clone)]
pub enum Message{
    Inc,
    Dec,
    OnDrag(i32),
}

pub fn view(m: &Model) -> Vec<Box<UIComponent>> {
    vec![
        ui::button(20, 20, 100, 30, "Inc".to_string(), Color::rgb(100, 100, 100), Some(Message::Inc)),
        ui::button(20, 60, 100, 30, "Dec".to_string(), Color::rgb(100, 100, 100), Some(Message::Dec)),
        ui::text_element(20, 100, m.counter.to_string()),
        ui::slider(150, 20, 40, 150, 0, 10, m.counter, Color::rgb(100, 100, 100), Color::rgb(255, 0, 0), |x| Message::OnDrag(x))
    ]
}

pub fn update(m: Model, msg: Message) -> Model{
    match msg {
        Message::Inc => Model{counter: m.counter+1, ..m},
        Message::Dec => Model{counter: m.counter-1, ..m},
        Message::OnDrag(x) => Model{counter: x, ..m},
    }
}