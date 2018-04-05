use stm32f7::lcd::{Layer, Framebuffer, Color};

pub fn draw_line<T: Framebuffer> (lcd: &mut Layer<T>, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
    let x0: i32 = x0 as i32;
    let x1: i32 = x1 as i32;
    let y0: i32 = y0 as i32;
    let y1: i32 = y1 as i32;
    
    let x_diff = (x0 - x1).abs();
    let y_diff = (y0 - y1).abs();

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
