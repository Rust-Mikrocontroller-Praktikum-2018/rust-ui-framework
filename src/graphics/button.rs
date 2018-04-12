use graphics::point::Point;
use graphics::ui_component::UIComponent;
use stm32f7::lcd::{Framebuffer, FramebufferAl88, FramebufferArgb8888};
use stm32f7::lcd::Layer;
use graphics::{Message, TouchEvent};
use stm32f7::lcd::Color;
use graphics::rectangle::Rectangle;
use alloc::String;

use core::any::Any;

pub struct Button {
    rectangle: Rectangle,
    text: String,
    on_click_message: Option<Message>,
    last_evt_pos: Point,
}

impl Button {
    pub fn new(left: usize, top: usize, width: usize, height: usize, text: String, color: Color, on_click_message: Option<Message>) -> Button {
        Button{
            rectangle: Rectangle::new(left, top, width, height, color, true),
            text,
            on_click_message,
            last_evt_pos: Point{x: 0, y:0},
        }
    }

    pub fn clear_text<V: Framebuffer>(&self, lcd_text: &mut Layer<V>) {
        let text_point = (
            self.rectangle.upper_left.x + 5,
            (self.rectangle.upper_left.y + self.rectangle.lower_right.y) / 2 - 7,
        );
        lcd_text
            .text_writer()
            .clear_str_at(text_point.0, text_point.1, &self.text);
    }

    fn write_text<V: Framebuffer>(&self, lcd_text: &mut Layer<V>) {
        let text_point = (
            self.rectangle.upper_left.x + 5,
            (self.rectangle.upper_left.y + self.rectangle.lower_right.y) / 2 - 7,
        );
        lcd_text
            .text_writer()
            .print_str_at(text_point.0, text_point.1, &self.text);
    }
}

impl UIComponent for Button {

    fn as_any(&self) -> &Any {
        self
    }

    fn clear(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>) {
        self.rectangle.clear(lcd_ui, lcd_text);
        self.clear_text(lcd_text);
    }

    fn draw(&self, old_widget: Option<&UIComponent>, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>){

        let old_button = match old_widget {
            Some(ow) => ow.as_any().downcast_ref::<Button>(),
            None => None,
        };

        match old_button {
            Some(o_w) => {

                //if position of text changes, clear old and draw new
                if o_w.rectangle.upper_left.x != self.rectangle.upper_left.x || o_w.rectangle.upper_left.y != self.rectangle.upper_left.y || o_w.rectangle.lower_right.y != self.rectangle.lower_right.y || o_w.text != self.text {
                    o_w.clear_text(lcd_text);
                    self.write_text(lcd_text);
                }
                self.rectangle.draw(Some(&o_w.rectangle), lcd_ui, lcd_text);
            },
            None => {
                if old_widget.is_some(){
                    old_widget.unwrap().clear(lcd_ui, lcd_text);
                }

                self.paint(lcd_ui, lcd_text);
            }
        }
    }

    fn is_in_bounding_box(&self, point: &Point) -> bool {
        if point.x < self.rectangle.upper_left.x || point.y < self.rectangle.upper_left.y {
            false
        } else if point.x > self.rectangle.lower_right.x || point.y > self.rectangle.lower_right.y {
            false
        } else {
            true
        }
    }

    fn on_touch(&mut self, evt: &TouchEvent) -> Option<Message>{
        match evt {
            &TouchEvent::Pressed(p) => {
                self.last_evt_pos = p;
                None
            },
            &TouchEvent::Moved(p) => {
                self.last_evt_pos = p;
                None
            },
            &TouchEvent::Released =>
                if self.is_in_bounding_box(&self.last_evt_pos) {
                    self.on_click_message.clone()
                }else{
                    None
                },
        }
    }

    fn paint(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>){
        self.write_text(lcd_text);
        self.rectangle.paint(lcd_ui, lcd_text);
    }
}
