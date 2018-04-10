use graphics::point::Point;
use graphics::rectangle;
use graphics::ui_component::UIComponent;
use stm32f7::lcd::{Framebuffer, FramebufferAl88, FramebufferArgb8888};
use stm32f7::lcd::Layer;
use graphics::{Message, TouchEvent};
use stm32f7::lcd::Color;

pub struct Button {
    pub upper_left: Point,
    pub lower_right: Point,
    pub text: &'static str,
    pub on_click_message: Option<Message>,
}

impl Button {
    pub fn new(left: usize, top: usize, width: usize, height: usize, text: &'static str, on_click_message: Option<Message>) -> Button {
        Button{
            upper_left: Point{x: left, y: top},
            lower_right: Point{x: left+width, y: top+height},
            text,
            on_click_message,
        }
    }
}

impl UIComponent for Button {

    fn get_message(&self) -> Option<Message>{
        self.on_click_message
    }

    fn on_touch(&mut self, evt: &TouchEvent) -> Option<Message>{
        match evt {
            &TouchEvent::Released => self.on_click_message.clone(),
            _ => None,
        }
    }

    fn draw(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>){
        rectangle::draw_rectangle(lcd_ui, &self.upper_left, &self.lower_right, Color::from_hex(0xffffff), true);
        let text_point = (self.upper_left.x + 5, (self.upper_left.y + self.lower_right.y)/2 - 7);
        lcd_text.text_writer().print_str_at(text_point.0, text_point.1, &self.text);
    }

    fn is_in_bounding_box(&self, point: &Point) -> bool {
        if point.x < self.upper_left.x || point.y < self.upper_left.y {
            false
        } else if point.x > self.lower_right.x || point.y > self.lower_right.y {
            false
        } else {
            true
        }
    }
}
