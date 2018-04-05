use stm32f7::lcd::{Layer, Framebuffer, Color};
use graphics::{line, point::Point};

pub fn draw_rectangle<T: Framebuffer> (lcd: &mut Layer<T>, p0: &Point, p1: &Point, color: Color, fill: bool) {
    if fill {
        for x in p0.x..p1.x {
            for y in p0.y..p1.y {
                lcd.print_pixel_color_at(x, y, color);
            }
        }
    } else {
        let points = [
            Point {
                x: p0.x,
                y: p0.y,
            },
            Point {
                x: p0.x,
                y: p1.y,
            },
            Point {
                x: p1.x,
                y: p1.y,
            },
            Point {
                x: p1.x,
                y: p0.y,
            },
        ];
    }
}