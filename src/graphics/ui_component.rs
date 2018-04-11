use stm32f7::lcd::{FramebufferArgb8888, FramebufferAl88};
use stm32f7::lcd::Layer;
//use lcd::Color;
use graphics::point::Point;
use graphics::{TouchEvent, Message};
use core::any::Any;

pub trait UIComponent : Any{

    fn is_in_bounding_box(&self, p: &Point) -> bool;

    fn on_touch(&mut self, evt: &TouchEvent) -> Option<Message>;

    fn draw(&self, old_widget: Option<&UIComponent>, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>);

    fn clear(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>);

    fn as_any(&self) -> &Any;
}
