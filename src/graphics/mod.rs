pub mod line;
pub mod point;
//pub mod polygon;
pub mod rectangle;
//pub mod circle;
pub mod button;
pub mod ui_component;

#[derive(Clone, Copy)]
pub enum Message{
    Increment,
    Decrement
}

pub enum TouchEvent{
    Pressed(point::Point),
    Moved(point::Point),
    Released,
}
