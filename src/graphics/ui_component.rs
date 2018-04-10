use stm32f7::lcd::Framebuffer;
use stm32f7::lcd::Layer;
use graphics::point::Point;

pub trait UIComponent {

    /**
     * Clears the component by repainting it with the background color.
     */
    fn clear<T: Framebuffer, V: Framebuffer> (&self, lcd_ui: &mut Layer<T>, lcd_text: &mut Layer<V>);

    fn is_in_bounding_box(&self, p: Point) -> bool;

    fn on_touch(&mut self, evt: TouchEvent) -> Option<Message>;

    fn draw<T: Framebuffer, V: Framebuffer>(&self, old_widget: Option<&Self>, lcd_ui: &mut Layer<T>, lcd_text: &mut Layer<V>);
}

pub trait Clickable {
    fn on_click(&mut self, m: Message);
}

pub enum TouchEvent{
    Pressed(Point),
    Moved(Point),
    Released,
}

pub enum Message{
    OnButtonClick,
    OnSomeButtonClick,
    OnMyCircleDrag(i32, i32)
}
/*
fn view(m: Model) -> Vec<UIComponent> {
    vec![Button(m.button_pos_x, 20, 40, 50, Color::rgb(255, 0, 0)).on_click(OnSomeButtonClick),
        Button(m.button_pos_x, 20, 40, 50, Color::rgb(255, 255, 0)).on_click(OnSomeButtonClick),
        Circle(10, 10, 10).on_drag(OnMyCircleDrag)
        ...]
}

fn update(msg: Message, m: Model) -> Model {
    match msg {
        OnSomeButtonClick => Model{
            button_pos_x += 10,
            ..m
        },
        OnMyCircleDrag(x, y) =>
    }
}*/