use graphics::model::Model;
use stm32f7::lcd::{Color, Framebuffer, Layer};
use graphics::circle;

pub fn view<T: Framebuffer>(m: &Model, lcd: &mut Layer<T>) {
//    lcd.clear();
    match m.cursor.first_contact {
        Some(ref p) => circle::draw_circle(lcd, &p, m.r, Color::from_hex(0xFFFFFF)),
        None => (),
    }

    match m.cursor.second_contact {
        Some(ref p) => circle::draw_circle(lcd, &p, 10, Color::from_hex(0xFF0000)),
        None => (),
    }

    match m.cursor.last_contact {
        Some(ref p) => circle::draw_circle(lcd, &p, m.r, Color::from_hex(0x00FF00)),
        None => (),
    }
    
}