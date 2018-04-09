use graphics::point::Point;
use lcd::Color;

trait Button {
    let upper_left: Point;
    let lower_right: Point;
    let text: str;

    fn button_action ();
}

impl UIComponent for Button {
    fn paint<T: Framebuffer> (lcd: &mut Layer<T>, fg: Color) {
        rectangle::draw(lcd, upper_left, lower_right, fg, false);
        // TODO: text
    }

    fn clear<T: Framebuffer> (lcd: &mut Layer<T>, bg: Color) {
        paint(lcd, bg);
    }

    fn click(point: Point) -> bool {
        if point.x < upper_left.x || point.y < upper_left.y {
            false
        } else if point.x > lower_right.x || point.y > lower_right.y {
            false
        } else {
            button_action();
            true
        }
    }
}
