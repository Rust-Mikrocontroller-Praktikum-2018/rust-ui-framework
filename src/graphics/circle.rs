use stm32f7::lcd::{Color, Framebuffer, FramebufferAl88, FramebufferArgb8888, Layer};
use graphics::point::Point;
use graphics::ui_component::UIComponent;
use graphics::{Message, TouchEvent};
use core::any::Any;

pub struct Circle {
    pub midpoint: Point,
    radius: i32,
    pub color: Color,
    filled: bool,
}

impl Circle {
    pub fn new(midpoint: Point, radius: i32, color: Color, filled: bool) -> Circle {
        Circle {
            midpoint,
            radius,
            color,
            filled,
        }
    }

    fn draw_cirle(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, transparent: bool, filled: bool) {
        let c: Color = if transparent {
            Color::rgba(0, 0, 0, 0)
        } else {
            self.color
        };
        let x0 = self.midpoint.x as i32;
        let y0 = self.midpoint.y as i32;
        let mut x: i32 = self.radius - 1;
        let mut y: i32 = 0;
        let mut dx: i32 = 1;
        let mut dy: i32 = 1;
        let mut err: i32 = dx - 2 * self.radius;

        loop {
            if x < y {
                break;
            } else {
                let start = if filled { y } else { x };
                for i in start..=x {
                    print_point_at(lcd_ui, x0 as i32 + i, y0 as i32 + y, c);
                    print_point_at(lcd_ui, x0 as i32 + y, y0 as i32 + i, c);
                    print_point_at(lcd_ui, x0 as i32 - y, y0 as i32 + i, c);
                    print_point_at(lcd_ui, x0 as i32 - i, y0 as i32 + y, c);
                    print_point_at(lcd_ui, x0 as i32 - i, y0 as i32 - y, c);
                    print_point_at(lcd_ui, x0 as i32 - y, y0 as i32 - i, c);
                    print_point_at(lcd_ui, x0 as i32 + y, y0 as i32 - i, c);
                    print_point_at(lcd_ui, x0 as i32 + i, y0 as i32 - y, c);
                }

                if err <= 0 {
                    y += 1;
                    err += dy;
                    dy += 2;
                }

                if err > 0 {
                    x -= 1;
                    dx += 2;
                    err += dx - 2 * self.radius;
                }
            }
        }
    }
}

impl UIComponent for Circle {
    fn as_any(&self) -> &Any {
        self
    }

    fn clear(
        &self,
        lcd_ui: &mut Layer<FramebufferArgb8888>,
        _lcd_text: &mut Layer<FramebufferAl88>,
    ) {
        self.draw_cirle(lcd_ui, true, self.filled);
    }

    fn draw(&self, old_widget: Option<&UIComponent>, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>) {
        let old_circ = match old_widget {
            Some(ow) => ow.as_any().downcast_ref::<Circle>(),
            None => None,
        };

        match old_circ {
            Some(o_w) => {
                //if midpoints are not equal or new circle is smaller than old or not filled, clear old circle and draw new. If midpoints are equal, some improvements are possible
                if o_w.midpoint.x != self.midpoint.x || o_w.midpoint.y != self.midpoint.y || o_w.radius > self.radius || !self.filled{
                    o_w.clear(lcd_ui, lcd_text);
                    self.paint(lcd_ui, lcd_text);
                } else 
                //if old radius is smaller than new one or colors differ, draw new circle without clearing old
                if o_w.radius < self.radius || o_w.color.red != self.color.red || o_w.color.blue != self.color.blue || o_w.color.green != self.color.green || o_w.color.alpha != self.color.alpha {
                    self.paint(lcd_ui, lcd_text);
                }
            }
            None => {
                if old_widget.is_some(){
                    old_widget.unwrap().clear(lcd_ui, lcd_text);
                }

                self.paint(lcd_ui, lcd_text);
            }
        }
    }

    fn is_in_bounding_box(&self, p: &Point) -> bool {
        return p.x as i32 <= self.midpoint.x as i32 + self.radius -1 && 
            p.x as i32 >= self.midpoint.x as i32 - self.radius + 1 &&
            p.y as i32 <= self.midpoint.y as i32 + self.radius -1 &&
            p.y as i32 >= self.midpoint.y as i32 - self.radius -1;
    }

    fn on_touch(&mut self, _evt: &TouchEvent) -> Option<Message>{
        None
    }

    fn paint(&self, lcd_ui: &mut Layer<FramebufferArgb8888>, _lcd_text: &mut Layer<FramebufferAl88>){
        self.draw_cirle(lcd_ui, false, self.filled);
    }
}

/* pub fn draw_filled_circle<T: Framebuffer>(
    lcd: &mut Layer<T>,
    m: &Point,
    r: i32,
    color: Color,
    filled: bool,
) {
    let x0 = m.x as i32;
    let y0 = m.y as i32;
    let mut x: i32 = r - 1;
    let mut y: i32 = 0;
    let mut dx: i32 = 1;
    let mut dy: i32 = 1;
    let mut err: i32 = dx - 2 * r;

    loop {
        if x < y {
            break;
        } else {
            let start = if filled { y } else { x };
            for i in start..=x {
                print_point_at(lcd, x0 as i32 + i, y0 as i32 + y, color);
                print_point_at(lcd, x0 as i32 + y, y0 as i32 + i, color);
                print_point_at(lcd, x0 as i32 - y, y0 as i32 + i, color);
                print_point_at(lcd, x0 as i32 - i, y0 as i32 + y, color);
                print_point_at(lcd, x0 as i32 - i, y0 as i32 - y, color);
                print_point_at(lcd, x0 as i32 - y, y0 as i32 - i, color);
                print_point_at(lcd, x0 as i32 + y, y0 as i32 - i, color);
                print_point_at(lcd, x0 as i32 + i, y0 as i32 - y, color);
            }

            if err <= 0 {
                y += 1;
                err += dy;
                dy += 2;
            }

            if err > 0 {
                x -= 1;
                dx += 2;
                err += dx - 2 * r;
            }
        }
    }
} */

fn print_point_at<T: Framebuffer>(lcd: &mut Layer<T>, a: i32, b: i32, color: Color) {
    if a < 480 && a >= 0 && b >= 0 && b < 272 {
        lcd.print_point_color_at(a as usize, b as usize, color);
    }
}
