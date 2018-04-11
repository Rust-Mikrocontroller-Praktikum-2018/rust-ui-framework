use stm32f7::lcd::{Color, Framebuffer, Layer};
use graphics::point::Point;

pub fn draw_line<T: Framebuffer>(lcd: &mut Layer<T>, p0: &Point, p1: &Point, color: Color) {
    let mut x0: i32 = p0.x as i32;
    let mut x1: i32 = p1.x as i32;
    let mut y0: i32 = p0.y as i32;
    let mut y1: i32 = p1.y as i32;

    let x_diff = (x0 - x1).abs();
    let y_diff = (y0 - y1).abs();

    if p0.x == p1.x && p0.y == p1.y {
        if p0.x >= 480 || p0.y >= 272 { // screen size: 480x272
            return;
        }
        lcd.print_point_color_at(p0.x, p0.y, color);
        return;
    }

    if x_diff > y_diff {
        if x0 > x1 {
            let x = x0;
            x0 = x1;
            x1 = x;
            let y = y0;
            y0 = y1;
            y1 = y;
        }
        for x in x0.max(0)..=x1.min(480-1) {
            let x = x - x0;
            let height = (y1 - y0) * x / (x1 - x0);
            lcd.print_point_color_at((x + x0) as usize, (y0 + height) as usize, color);
        }
    } else {
        if y0 > y1 {
            let x = x0;
            x0 = x1;
            x1 = x;
            let y = y0;
            y0 = y1;
            y1 = y;
        }
        for y in y0.max(0)..=y1.min(272-1) {
            let y = y - y0;
            let height = (x1 - x0) * y / (y1 - y0);
            lcd.print_point_color_at((x0 + height) as usize, (y + y0) as usize, color);
        }
    }
}
