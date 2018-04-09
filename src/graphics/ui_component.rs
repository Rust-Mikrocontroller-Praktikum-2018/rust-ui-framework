trait UIComponent {
    /**
     * Draws the component to the layer with given foreground color.
     */
    fn paint<T: Framebuffer> (lcd: &mut Layer<T>, fg: Color);
    /**
     * Clears the component by repainting it with the background color.
     */
    fn clear<T: Framebuffer> (lcd: &mut Layer<T>, bg: Color);
    /**
     * Receives the filtered click position and executes on hit.
     * If the point hits the component, it performes its action and return true.
     * Returns false otherwise.
     */
    fn click(point: Point) -> bool;
}