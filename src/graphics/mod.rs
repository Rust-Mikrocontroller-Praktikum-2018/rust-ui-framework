
pub mod point;
pub mod ui_component;

mod line;
mod polygon;
mod rectangle;
mod circle;
mod button;
mod slider;

#[derive(Clone, Copy)]
pub enum Message{
    Increment,
    Decrement,
    ToMenuScreen,
    ToWidgetsScreen,
    ToDotScreen,
    ToColorScreen,
    OnChange(i32),
}

pub enum TouchEvent{
    Pressed(point::Point),
    Moved(point::Point),
    Released,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Screen{
    Menu,
    Dot,
    Color,
    Keyboard,
    Widgets,
}

pub mod ui{
    use alloc::{Vec, String, boxed::Box};
    use graphics::button::Button;
    use graphics::rectangle::Rectangle;
    use graphics::slider::Slider;
    use graphics::circle::Circle;
    use graphics::polygon::Polygon;
    use graphics::point::Point;
    use stm32f7::lcd::Color;
    use graphics::Message;

    pub fn button(left: usize, top: usize, width: usize, height: usize, text: String, color: Color, on_click_message: Option<Message>) -> Box<Button>{
        Box::new(Button::new(left, top, width, height, text, color, on_click_message))
    }

    pub fn rectangle(left: usize, top: usize, width: usize, height: usize, color: Color, filled: bool) -> Box<Rectangle>{
        Box::new(Rectangle::new(left, top, width, height, color, filled))
    }

    pub fn circle(x: usize, y: usize, radius: i32, color: Color, filled: bool) -> Box<Circle> {
        Box::new(Circle::new(x, y, radius, color, filled))
    }

    pub fn polygon(points: Vec<Point>, color: Color, filled: bool) -> Box<Polygon> {
        Box::new(Polygon::new(points, color, filled))
    }

    pub fn slider<F: Fn(i32) -> Message>(left: usize, top: usize, width: usize, height: usize, min_value: i32, max_value: i32, initial_value: i32, bg_color: Color, fg_color: Color, on_drag_message: F) -> Box<Slider<F>> {
        Box::new(Slider::new(left, top, width, height, min_value, max_value, initial_value, bg_color,fg_color, on_drag_message))
    }

    pub fn point(x: usize, y: usize) -> Point {
        Point{x, y}
    }
}