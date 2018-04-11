use stm32f7::lcd::{Layer, FramebufferArgb8888, FramebufferAl88};
use graphics::{point::Point, line};
use stm32f7::lcd::Color;
use graphics::ui_component::UIComponent;
use graphics::{TouchEvent, Message};
use core::any::Any;


pub struct Rectangle {
    upper_left: Point,
    lower_right: Point,
    color: Color,
}

impl Rectangle {
    pub fn new(left: usize, top: usize, width: usize, height: usize, color: Color) -> Rectangle {
        Rectangle {
            upper_left: Point { x: left, y: top },
            lower_right: Point { x: left + width, y: top + height },
            color,
        }
    }

    fn paint(&self, lcd_ui: &mut Layer<FramebufferArgb8888>) {
        self.paint_with_color(lcd_ui, self.color);
    }

    fn paint_transparent(&self, lcd_ui: &mut Layer<FramebufferArgb8888>) {
        self.paint_with_color(lcd_ui, Color::rgba(0, 0, 0, 0));
    }

    fn paint_with_color(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, color: Color) {
        draw_rectangle(lcd_ui, &self.upper_left, &self.lower_right, color, true);
    }
}

impl UIComponent for Rectangle {
    fn as_any(&self) -> &Any { self }

    fn clear(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, _lcd_text: &mut Layer<FramebufferAl88>) {
        self.paint_transparent(lcd_ui);
    }

    fn draw(&self, old_widget: Option<&UIComponent>, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>){
        let bg = Color {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 0,
        };

        let old_rect = match old_widget {
            Some(ow) => ow.as_any().downcast_ref::<Rectangle>(),
            None => None,
        };

        match old_rect {
            Some(o_w) =>
                // if old and new don't lay over each other, clear old and draw new
                if o_w.lower_right.x <= self.upper_left.x || o_w.lower_right.y <= self.upper_left.y || o_w.upper_left.x >= self.lower_right.x ||
                o_w.upper_left.y >= self.lower_right.y {
                    o_w.clear(lcd_ui, lcd_text);
                    draw_rectangle(lcd_ui, &self.upper_left, &self.lower_right, self.color, true);
                } else {

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
                     draw_rectangle(lcd_ui, &o_w.upper_left, &Point{x: self.upper_left.x, y: o_w.lower_right.y}, bg, true);
                 } else if o_w.upper_left.x > self.upper_left.x {
                     // draw left part of new rectangle
                     draw_rectangle(lcd_ui, &self.upper_left, &Point{x: o_w.upper_left.x, y: self.lower_right.y}, self.color, true);
                 }

                 if o_w.upper_left.y < self.upper_left.y {
                        //delete upper part of old rectangle (which wasn't deleted before)
                        draw_rectangle(lcd_ui, &Point{x:max_x_upper_left, y:o_w.upper_left.y}, &Point{x: o_w.lower_right.x, y:self.upper_left.y}, bg, true);
                 } else if o_w.upper_left.y > self.upper_left.y {
                        //draw upper part of new rectangle (which wasn't drawn before)
                        draw_rectangle(lcd_ui, &Point{x:max_x_upper_left, y:self.upper_left.y}, &Point{x: self.lower_right.x, y:o_w.upper_left.y}, self.color, true);
                 }

                 if o_w.lower_right.x < self.lower_right.x {
                        //draw right part of new rectangle (which wasn't drawn before)
                        draw_rectangle(lcd_ui, &Point{x:o_w.lower_right.x, y:max_y_upper_left}, &self.lower_right, self.color, true);
                 } else if o_w.lower_right.x > self.lower_right.x {
                        //delete right part of old rectangle (which wasn't deleted before)
                        draw_rectangle(lcd_ui, &Point{x:self.lower_right.x, y:max_y_upper_left}, &o_w.lower_right, bg, true);
                 }

                 if o_w.lower_right.y < self.lower_right.y {
                        //draw lower part of new rectangle (which wasn't drawn before)
                        draw_rectangle(lcd_ui, &Point{x:max_x_upper_left, y: o_w.lower_right.y}, &Point{x:min_x_lower_right, y:self.lower_right.y}, self.color, true);
                 } else if o_w.lower_right.y > self.lower_right.y {
                        //delete lower part of old rectangle (which wasn't deleted before)
                        draw_rectangle(lcd_ui, &Point{x:max_x_upper_left, y: self.lower_right.y}, &Point{x:min_x_lower_right, y:o_w.lower_right.y}, bg, true);
                 }

                } else {
                 draw_rectangle(
                    lcd_ui,
                    &o_w.upper_left,
                    &o_w.lower_right,
                    bg,
                    true,
                 );
                 self.paint(lcd_ui);
                }
                },
            None => {
                if old_widget.is_some(){
                    old_widget.unwrap().clear(lcd_ui, lcd_text);
                }

                self.paint(lcd_ui);
            }
        }
    }

    fn is_in_bounding_box(&self, p: &Point) -> bool {
        return p.x >= self.upper_left.x &&
            p.x <= self.lower_right.x &&
            p.y >= self.upper_left.y &&
            p.y <= self.lower_right.y;
    }

    fn on_touch(&mut self, _evt: &TouchEvent) -> Option<Message>{
        None
    }
}

// TODO: make private
pub fn draw_rectangle(lcd_ui: &mut Layer<FramebufferArgb8888>, ul: &Point, lr: &Point, color: Color, fill: bool){
    if fill {
        for y in ul.y.min(271)..=lr.y.min(271) {
            for x in ul.x.min(479)..=lr.x.min(479) {
                lcd_ui.print_point_color_at(x, y, color);
            }
        }
    } else {
        let points = [
            Point {
                x: ul.x,
                y: ul.y,
            },
            Point {
                x: ul.x,
                y: lr.y,
            },
            Point {
                x: lr.x,
                y: lr.y,
            },
            Point {
                x: lr.x,
                y: ul.y,
            },
        ];
        let mut last_point = &points[points.len() - 1];
        for p in points.iter() {
            line::draw_line(lcd_ui, last_point, p, color);
            last_point = p;
        }
    }
}