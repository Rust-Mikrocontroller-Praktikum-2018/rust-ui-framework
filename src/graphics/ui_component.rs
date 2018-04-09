use stm32f7::lcd::Framebuffer;
use stm32f7::lcd::Layer;
use lcd::Color;
use graphics::point::Point;

pub trait UIComponent {
    /**
     * Draws the component to the layer with given foreground color.
     */
    fn paint<T: Framebuffer, V: Framebuffer> (&self, lcd_ui: &mut Layer<T>, lcd_text: &mut Layer<V>, fg: Color);
    /**
     * Clears the component by repainting it with the background color.
     */
    fn clear<T: Framebuffer, V: Framebuffer> (&self, lcd_ui: &mut Layer<T>, lcd_text: &mut Layer<V>, bg: Color);
    /**
     * Receives the filtered click position and executes on hit.
     * If the point hits the component, it performes its action and return true.
     * Returns false otherwise.
     */
    fn click(&self, point: Point) -> bool;
}