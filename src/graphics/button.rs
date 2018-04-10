use graphics::point::Point;
use graphics::rectangle;
use graphics::ui_component::UIComponent;
use graphics::ui_component::TouchEvent;
use graphics::ui_component::Message;
use stm32f7::lcd::Framebuffer;
use stm32f7::lcd::Layer;
use lcd::Color;

pub struct Button {
    pub upper_left: Point,
    pub lower_right: Point,
    pub text: &'static str,
    pub color: Color,
}

impl Button {
    pub fn clear_text<V: Framebuffer>(&self, lcd_text: &mut Layer<V>) {
        let text_point = (
            self.upper_left.x + 5,
            (self.upper_left.y + self.lower_right.y) / 2 - 7,
        );
        lcd_text
            .text_writer()
            .clear_str_at(text_point.0, text_point.1, &self.text);
    }

    pub fn write_text<V: Framebuffer>(&self, lcd_text: &mut Layer<V>) {
        let text_point = (
            self.upper_left.x + 5,
            (self.upper_left.y + self.lower_right.y) / 2 - 7,
        );
        lcd_text
            .text_writer()
            .print_str_at(text_point.0, text_point.1, &self.text);
    }
}

impl UIComponent for Button {
    fn draw<T: Framebuffer, V: Framebuffer>(
        &self,
        old_widget: Option<&Self>,
        lcd_ui: &mut Layer<T>,
        lcd_text: &mut Layer<V>,
    ) {
        let bg = Color {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 0,
        };
        match old_widget {
            Some(o_w) => 
                // if old and new don't lay over each other, clear old and draw new
                if o_w.lower_right.x <= self.upper_left.x || o_w.lower_right.y <= self.upper_left.y || o_w.upper_left.x >= self.lower_right.x ||
                o_w.upper_left.y >= self.lower_right.y {
                    o_w.clear(lcd_ui, lcd_text);
                    rectangle::draw_rectangle(lcd_ui, &self.upper_left, &self.lower_right, self.color, true,);
                    self.write_text(lcd_text);
                } else {

                //if position of text changes, clear old and draw new
                 if o_w.upper_left.x != self.upper_left.x || o_w.upper_left.y != self.upper_left.y || o_w.lower_right.y != self.lower_right.y {
                    o_w.clear_text(lcd_text);
                    self.write_text(lcd_text);
                 }
                
                //if colors of o_w and self are equal, start comparing positions
                if o_w.color.red == self.color.red && o_w.color.blue == self.color.blue && o_w.color.green == self.color.green && o_w.color.alpha == self.color.alpha  {
                 
                 let max_x_upper_left : usize;
                        if o_w.upper_left.x >= self.upper_left.x {
                            max_x_upper_left = o_w.upper_left.x;
                        } else {
                            max_x_upper_left = self.upper_left.x;
                        };
                 let max_y_upper_left : usize;
                        if o_w.upper_left.y >= self.upper_left.y {
                            max_y_upper_left = o_w.upper_left.y;
                        } else {
                            max_y_upper_left = self.upper_left.y;
                        };
                 let min_x_lower_right : usize;
                        if o_w.lower_right.x <= self.lower_right.x {
                            min_x_lower_right = o_w.lower_right.x;
                        } else {
                            min_x_lower_right = self.lower_right.x;
                        }

                 if o_w.upper_left.x < self.upper_left.x {
                     //delete left part of old rectangle
                     rectangle::draw_rectangle(lcd_ui, &o_w.upper_left, &Point{x: self.upper_left.x, y: o_w.lower_right.y}, bg, true);
                 } else if o_w.upper_left.x > self.upper_left.x {
                     // draw left part of new rectangle
                     rectangle::draw_rectangle(lcd_ui, &self.upper_left, &Point{x: o_w.upper_left.x, y: self.lower_right.y}, self.color, true);
                 }

                 if o_w.upper_left.y < self.upper_left.y {
                        //delete upper part of old rectangle (which wasn't deleted before)
                        rectangle::draw_rectangle(lcd_ui, &Point{x:max_x_upper_left, y:o_w.upper_left.y}, &Point{x: o_w.lower_right.x, y:self.upper_left.y}, bg, true);
                 } else if o_w.upper_left.y > self.upper_left.y {
                        //draw upper part of new rectangle (which wasn't drawn before)
                        rectangle::draw_rectangle(lcd_ui, &Point{x:max_x_upper_left, y:self.upper_left.y}, &Point{x: self.lower_right.x, y:o_w.upper_left.y}, self.color, true);
                 }

                 if o_w.lower_right.x < self.lower_right.x {
                        //draw right part of new rectangle (which wasn't drawn before)
                        rectangle::draw_rectangle(lcd_ui, &Point{x:o_w.lower_right.x, y:max_y_upper_left}, &self.lower_right, self.color, true);
                 } else if o_w.lower_right.x > self.lower_right.x {
                        //delete right part of old rectangle (which wasn't deleted before)
                        rectangle::draw_rectangle(lcd_ui, &Point{x:self.lower_right.x, y:max_y_upper_left}, &o_w.lower_right, bg, true);
                 }

                 if o_w.lower_right.y < self.lower_right.y {
                        //draw lower part of new rectangle (which wasn't drawn before)
                        rectangle::draw_rectangle(lcd_ui, &Point{x:max_x_upper_left, y: o_w.lower_right.y}, &Point{x:min_x_lower_right, y:self.lower_right.y}, self.color, true);
                 } else if o_w.lower_right.y > self.lower_right.y {
                        //delete lower part of old rectangle (which wasn't deleted before)
                        rectangle::draw_rectangle(lcd_ui, &Point{x:max_x_upper_left, y: self.lower_right.y}, &Point{x:min_x_lower_right, y:o_w.lower_right.y}, bg, true);
                 }
                
                } else {
                 rectangle::draw_rectangle(
                    lcd_ui,
                    &o_w.upper_left,
                    &o_w.lower_right,
                    bg,
                    true,
                 );
                 rectangle::draw_rectangle(
                    lcd_ui,
                    &self.upper_left,
                    &self.lower_right,
                    self.color,
                    true,
                 );
                }
                },
            None => {
                rectangle::draw_rectangle(
                    lcd_ui,
                    &self.upper_left,
                    &self.lower_right,
                    self.color,
                    true,
                );
                self.write_text(lcd_text);
            }
        }
    }

    fn clear<T: Framebuffer, V: Framebuffer>(
        &self,
        lcd_ui: &mut Layer<T>,
        lcd_text: &mut Layer<V>,
    ) {
        let bg = Color {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 0,
        };
        rectangle::draw_rectangle(lcd_ui, &self.upper_left, &self.lower_right, bg, true);
        self.clear_text(lcd_text);
    }

    fn is_in_bounding_box(&self, point: Point) -> bool {
        if point.x < self.upper_left.x || point.y < self.upper_left.y {
            false
        } else if point.x > self.lower_right.x || point.y > self.lower_right.y {
            false
        } else {
            true
        }
    }

    fn on_touch(&mut self, evt: TouchEvent) -> Option<Message> {
        match evt {
            TouchEvent::Pressed(p) => if self.is_in_bounding_box(p) {
                Some(Message::OnButtonClick)
            } else {
                None
            },
            TouchEvent::Moved(p) => None,
            TouchEvent::Released => None,
        }
    }
}
