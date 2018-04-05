use stm32f7::lcd::{Color, Framebuffer, Layer};
use graphics::point::Point;

pub fn draw_line<T: Framebuffer>(lcd: &mut Layer<T>, p0: &Point, p1: &Point, color: Color) {
    let x0: i32 = p0.x as i32;
    let x1: i32 = p1.x as i32;
    let y0: i32 = p0.y as i32;
    let y1: i32 = p1.y as i32;

    let x_diff = (x0 - x1).abs();
    let y_diff = (y0 - y1).abs();

    if p0.x == p1.x && p0.y == p1.y {
        lcd.print_point_color_at(p0.x, p1.x, color);
    }

    if x_diff > y_diff {
        for x in x0..=x1 {
            let x = x - x0;
            let height = (y1 - y0) * x / (x1 - x0);
            lcd.print_point_color_at(x as usize, (y0 + height) as usize, color);
        }
    } else {
        for y in y0..=y1 {
            let y = y - y0;
            let height = (x1 - x0) * y / (y1 - y0);
            lcd.print_point_color_at(y as usize, (x0 + height) as usize, color);
        }
    }
}
