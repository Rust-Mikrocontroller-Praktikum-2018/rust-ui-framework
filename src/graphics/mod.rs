pub mod line;
pub mod point;
pub mod polygon;
pub mod rectangle;
pub mod circle;
pub mod button;
pub mod ui_component;
pub mod slider;
pub mod text_element;

#[derive(Clone, Copy)]
pub enum Message{
    CircleLeft,
    CircleRight,
    CircleUp,
    CircleDown,
    CircleInlarge,
    CircleDecrease,
    Increment,
    Decrement,
    OnChange(i32),
}

pub enum TouchEvent{
    Pressed(point::Point),
    Moved(point::Point),
    Released,
}

pub enum Screen{
    Menu,
    Dot,
    Color,
    Keyboard,
}