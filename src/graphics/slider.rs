use lcd::{Layer, FramebufferArgb8888, FramebufferAl88, Color};
use graphics::{point::Point, rectangle};
use graphics::{ui_component::UIComponent, TouchEvent, Message};
use core::any::Any;

pub struct Slider<F: Fn(i32) -> Message> {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    min_value: i32,
    max_value: i32,
    value: i32,
    bg_color: Color,
    fg_color: Color,
    message: F,
}

impl<F: Fn(i32) -> Message> Slider<F> {
    pub fn new(left: usize, top: usize, width: usize, height: usize, min_value: i32, max_value: i32, initial_value: i32, bg_color: Color, fg_color: Color, on_drag_message: F) -> Slider<F> {
        Slider{
            left: left,
            top: top,
            width: width,
            height: height,
            min_value: min_value,
            max_value: max_value,
            value: initial_value,
            bg_color: bg_color,
            fg_color: fg_color,
            message: on_drag_message,
        }
    }
}

impl<F: Fn(i32) -> Message + 'static> UIComponent for Slider<F> {

    fn as_any(&self) -> &Any {
        self
    }

    fn clear(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, _lcd_text: &mut Layer<FramebufferAl88>) {
        let lower_right = Point {
            x: self.left + self.width,
            y: self.top + self.height,
        };
        rectangle::draw_rectangle(lcd_ui, &Point{x: self.left, y: self.top}, &lower_right, Color::rgba(0, 0, 0, 0), true);
    }

    fn is_in_bounding_box(&self, p: &Point) -> bool {
        if p.x < self.left || p.y < self.top {
            false
        } else if p.x <= self.left + self.width && p.y <= self.top + self.height {
            true
        } else {
            false
        }
    }

    fn on_touch(&mut self, evt: &TouchEvent) -> Option<Message> {
        match *evt {
            TouchEvent::Pressed(_p) => {
                None
            },
            TouchEvent::Moved(p) => {
                let new_value: i32 = self.max_value - (p.y as i32 - self.top as i32) * (self.max_value - self.min_value) / self.height as i32;
                let new_value = new_value.max(self.min_value).min(self.max_value);
                Some((self.message)(new_value))
            },
            TouchEvent::Released => {
                // values stored, nothing to do
                None
            },
        }
    }

    fn paint(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, _lcd_text: &mut Layer<FramebufferAl88>) {
        let status_height: i32 = self.height as i32 * self.value / (self.max_value - self.min_value);
        let lower_right_bg = Point {
            x: self.left + self.width,
            y: self.top + self.height - (status_height as usize + 1),
        };
        let upper_left_fg = Point {
            x: self.left,
            y: lower_right_bg.y + 1,
        };
        let lower_right_fg = Point {
            x: self.left + self.width,
            y: self.top + self.height,
        };
        rectangle::draw_rectangle(lcd_ui, &Point{x: self.left, y: self.top}, &lower_right_bg, self.bg_color, true);
        rectangle::draw_rectangle(lcd_ui, &upper_left_fg, &lower_right_fg, self.fg_color, true);
    }
}
