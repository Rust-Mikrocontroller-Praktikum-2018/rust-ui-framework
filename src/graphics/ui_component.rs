use stm32f7::lcd::{Framebuffer, FramebufferArgb8888, FramebufferAl88};
use stm32f7::lcd::Layer;
use lcd::Color;
use graphics::point::Point;
use graphics::{TouchEvent, Message};

pub trait UIComponent {

    fn is_in_bounding_box(&self, p: &Point) -> bool;

    fn on_touch(&mut self, evt: &TouchEvent) -> Option<Message>;

    fn get_message(&self) -> Option<Message>;

    fn draw(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>);
}
