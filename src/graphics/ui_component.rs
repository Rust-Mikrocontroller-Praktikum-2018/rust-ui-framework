use stm32f7::lcd::{FramebufferArgb8888, FramebufferAl88};
use stm32f7::lcd::Layer;
//use lcd::Color;
use graphics::point::Point;
use graphics::{TouchEvent, gui::Message};
use core::any::Any;

pub trait UIComponent : Any{

    fn as_any(&self) -> &Any;

    fn clear(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>);

    fn draw(&self, old_widget: Option<&UIComponent>, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>){
        match old_widget {
            Some(w) => w.clear(lcd_ui, lcd_text),
            _ => ()
        }
        self.paint(lcd_ui, lcd_text);
    }

    fn is_in_bounding_box(&self, p: &Point) -> bool;

    fn on_touch(&mut self, evt: &TouchEvent) -> Option<Message>;

    fn paint(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>);
}
