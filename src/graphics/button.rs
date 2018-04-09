use graphics::point::Point;
use graphics::rectangle;
use graphics::ui_component::UIComponent;
use stm32f7::lcd::Framebuffer;
use stm32f7::lcd::Layer;
use lcd::Color;

pub struct Button {
    pub upper_left: Point,
    pub lower_right: Point,
    pub text: &'static str,
}

// impl Button {
//     pub fn button_action() {
//     }
// }

impl UIComponent for Button {
    fn paint<T: Framebuffer, V: Framebuffer> (&self, lcd_ui: &mut Layer<T>, lcd_text: &mut Layer<V>, fg: Color) {
        rectangle::draw_rectangle(lcd_ui, &self.upper_left, &self.lower_right, fg, true);
        let text_point = (self.upper_left.x + 5, (self.upper_left.y + self.lower_right.y)/2 - 7);
        lcd_text.text_writer().print_str_at(text_point.0, text_point.1, &self.text);
    }

    fn clear<T: Framebuffer, V: Framebuffer> (&self, lcd_ui: &mut Layer<T>, lcd_text: &mut Layer<V>, bg: Color) {
        self.paint(lcd_ui, lcd_text, bg);
    }

    fn click(&self, point: Point) -> bool {
        if point.x < self.upper_left.x || point.y < self.upper_left.y {
            false
        } else if point.x > self.lower_right.x || point.y > self.lower_right.y {
            false
        } else {
            // self.button_action();
            true
        }
    }
}
