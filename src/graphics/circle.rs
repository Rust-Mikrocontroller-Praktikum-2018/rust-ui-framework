use stm32f7::lcd::{Color, Framebuffer, Layer};
use graphics::point::Point;

pub fn draw_filled_circle<T: Framebuffer>(lcd: &mut Layer<T>,m: &Point, r : i32, color: Color, filled: bool) {
    let x0 = m.x as i32;
    let y0 = m.y as i32;
    let mut x: i32 = r-1;
    let mut y: i32 = 0;
    let mut dx : i32 = 1;
    let mut dy : i32 =1;
    let mut err :i32 = dx - 2*r;

    loop{
        if x < y {
            break;
        } else {
            let start = if filled {y} else {x};
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
                err += dx - 2*r;
            }
        }
    }

}

fn print_point_at<T: Framebuffer>(lcd: &mut Layer<T>,a : i32, b : i32, color: Color) {
    if a < 480 && a >= 0 && b >= 0 && b < 272 {
         lcd.print_point_color_at(a as usize, b as usize, color);
    }
}