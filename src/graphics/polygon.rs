use stm32f7::lcd::{Layer, Framebuffer, Color};
use graphics::{line, point::Point};

pub fn draw_polygon<T: Framebuffer> (lcd: &mut Layer<T>, points: &[Point], color: Color) {
    assert!(points.len() > 1);

    let last_point = &points[points.len()-1];
    for point in points {
        line::draw_line(lcd, last_point, point, color);
    }
}
