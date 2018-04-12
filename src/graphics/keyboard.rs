use graphics::button::Button;
use graphics::ui_component::UIComponent;
use graphics::point::Point;
use graphics::Message;
use graphics::TouchEvent;
use lcd::{Color, Layer, FramebufferArgb8888, FramebufferAl88};
use core::any::Any;
use alloc::Vec;
use alloc::{String, string::ToString};

pub struct Keyboard {
    buttons: Vec<Button>,
    text: String,
    color: Color,
}

impl Keyboard {
    pub fn new(initial_text: String, color: Color) -> Keyboard {
        Keyboard {
            buttons: Keyboard::get_keyboard_button_vector(color),
            text: initial_text,
            color: color,
        }
    }

    fn get_keyboard_button_vector(color: Color) -> Vec<Button> {
        let line_qwerty = vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p',];
        let line_asdf = vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l',];
        let line_zxc = vec!['z', 'x', 'c', 'v', 'b', 'n', 'm',];

        let button_size = 40;
        let spacer = 7;
        let mut button_left;
        let mut button_top = 0;

        let mut vector = vec![];

        for line in vec![line_zxc, line_asdf, line_qwerty,] {
            button_top += button_size + spacer;
            button_left = (480 - line.len() * (button_size + spacer) + spacer) / 2;
            for c in line {
                vector.push(Button::new(button_left, button_top, button_size, button_size, c.to_string(), color, Some(Message::KeyboardButtonMessage(c))));
                button_left += button_size + spacer;
            }
        }

        vector
    }

    fn get_button_index(&self, p: &Point) -> i32 {
        for (i, b) in self.buttons.iter().enumerate() {
            if b.is_in_bounding_box(p) {
                return i as i32;
            }
        }
        -1
    }
}

impl UIComponent for Keyboard {
    fn as_any(&self) -> &Any {
        self
    }

    fn draw(&self, old_widget: Option<&UIComponent>, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>){
        let old_kb = match old_widget {
            Some(ow) => ow.as_any().downcast_ref::<Keyboard>(),
            None => None,
        };

        match old_kb {
            Some(ow) => {
                if self.buttons.len() == ow.buttons.len() {
                    for (i, b) in self.buttons.iter().enumerate() {
                        b.draw(Some(&ow.buttons[i]), lcd_ui, lcd_text);
                    }
                } else {
                    for b in &ow.buttons {
                        b.clear(lcd_ui, lcd_text);
                    }
                    for b in &self.buttons {
                        b.paint(lcd_ui, lcd_text);
                    }
                }
            },
            None => {
                if old_widget.is_some(){
                    old_widget.unwrap().clear(lcd_ui, lcd_text);
                }

                self.paint(lcd_ui, lcd_text);
            },
        };
    }

    fn clear(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>) {
        for b in &self.buttons {
            b.clear(lcd_ui, lcd_text);
        }
    }

    fn is_in_bounding_box(&self, p: &Point) -> bool {
        self.get_button_index(p) != -1
    }

    fn on_touch(&mut self, evt: &TouchEvent) -> Option<Message> {
        match *evt {
            TouchEvent::Pressed(p) => {
                let pos = self.get_button_index(&p);
                if pos == -1 {
                    None
                } else {
                    let button_index = self.get_button_index(&p) as usize;
                    self.buttons[button_index].on_touch(evt)
                }
            },
            TouchEvent::Moved(p) => {
                let pos = self.get_button_index(&p);
                if pos == -1 {
                    None
                } else {
                    let button_index = self.get_button_index(&p) as usize;
                    self.buttons[button_index].on_touch(evt)
                }
            },
            TouchEvent::Released => None, // put point in released-event
        }
    }

    fn paint(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>) {
        for b in &self.buttons {
            b.paint(lcd_ui, lcd_text);
        }
    }
}
