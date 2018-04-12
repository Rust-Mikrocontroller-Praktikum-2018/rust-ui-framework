use graphics::{ui, ui_component::UIComponent};
use alloc::{Vec, String, boxed::Box, string::ToString};
use stm32f7::lcd::Color;

pub struct Model {
    pub screen: Screen,
    keyboard_text: String,
    position_circle_x: usize,
    position_circle_y: usize,
    radius_circle: i32,
    color: Color,
    dot_is_rec: bool,
}

pub fn generate_model() -> Model {
    Model{
        screen: Screen::Menu, 
        position_circle_x: 360, 
        position_circle_y: 135, 
        radius_circle: 30, 
        color: Color::rgb(255, 0, 100), 
        dot_is_rec: false, 
        keyboard_text: "".to_string(),
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Screen{
    Menu,
    Dot,
    Color,
    Keyboard,
    Widgets,
}

#[derive(Clone)]
pub enum Message{
    CircleLeft,
    CircleRight,
    CircleUp,
    CircleDown,
    CircleInlarge,
    CircleDecrease,
    CircleRectangle,
    ToMenuScreen,
    ToWidgetsScreen,
    ToDotScreen,
    ToColorScreen,
    ToKeyboardScreen,
    KeyboardButtonMessage(char),
    ColorRed(i32),
    ColorGreen(i32),
    ColorBlue(i32),
}

pub fn view(m: &Model) -> Vec<Box<UIComponent>> {
    let menu_button = ui::button(430, 20, 30, 30, " X".to_string(), Color::rgb(100, 100, 100), Some(Message::ToMenuScreen));

    match m.screen {
        Screen::Menu => {
            vec![
                ui::button(20, 20, 150, 30, "Widgets Demo".to_string(), Color::rgb(100, 100, 100), Some(Message::ToWidgetsScreen)),
                ui::button(20, 60, 150, 30, "Color Picker Demo".to_string(), Color::rgb(100, 100, 100), Some(Message::ToColorScreen)),
                ui::button(20, 100, 150, 30, "Dot Demo".to_string(), Color::rgb(100, 100, 100), Some(Message::ToDotScreen)),
                ui::button(20, 140, 150, 30, "Keyboard Demo".to_string(), Color::rgb(100, 100, 100), Some(Message::ToKeyboardScreen)),
            ]
        }
        Screen::Widgets => {
            let ll = ui::point(300, 250);
            let lr = ui::point(350, 250);
            let ul = ui::point(300, 200);
            let ur = ui::point(350, 200);
            let top = ui::point(325, 170);
            let house_points = vec![ll, ul, top, ur, lr, ul, ur, ll, lr];
            let star = vec![ui::point(325, 40),
                            ui::point(335, 80),
                            ui::point(375, 90),
                            ui::point(335, 100),
                            ui::point(325, 140),
                            ui::point(315, 100),
                            ui::point(275, 90),
                            ui::point(315, 80),
                            ];
            vec![ui::button(20, 50, 100, 30, "Button 1".to_string(), Color::rgb(100, 100, 100), None),
                    ui::button(20, 100, 100, 30, "Button 2".to_string(), Color::rgb(0, 150, 0), None),
                    ui::circle(70, 200, 50, Color::rgb(50, 50, 255), true),
                    ui::circle(200, 80, 30, Color::rgb(255, 0, 0), false),
                    ui::rectangle(170, 150, 30, 30, Color::rgb(255, 255, 0), false),
                    ui::rectangle(170, 200, 30, 30, Color::rgb(255, 255, 0), true),
                    ui::polygon(star, Color::from_hex(0xff00ff), true),
                    ui::polygon(house_points, Color::from_hex(0x00ffff), false),
                    menu_button,
                    ui::slider(430, 100, 30, 150, 0, 100, 75, Color::from_hex(0x333333), Color::from_hex(0xffffff), |x| Message::ColorRed(x)),
            ]
        }
        Screen::Dot => {
            let button_heigt = 30;
            let border = 20;
            let button_width = 50;
            let moved_element :Box<UIComponent> = if !m.dot_is_rec {
                ui::circle(m.position_circle_x, m.position_circle_y, m.radius_circle, m.color, true)
            } else {
                ui::rectangle(m.position_circle_x - m.radius_circle as usize, m.position_circle_y - m.radius_circle as usize, (m.radius_circle * 2) as usize, (m.radius_circle * 2) as usize, m.color, true)
            };
            let change_button = if !m.dot_is_rec {
                ui::button(border, border * 5 + button_heigt * 4, border * 2 + button_width * 3, button_heigt, "circle to rectangle".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleRectangle))
            } else {
                ui::button(border, border * 5 + button_heigt * 4, border * 2 + button_width * 3, button_heigt, "rectangle to circle".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleRectangle))
            };
            vec![ui::button(border, border *2 + button_heigt, button_width, button_heigt, "left".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleLeft)),
                    ui::button(border *2 + button_width, border, button_width, button_heigt, "up".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleUp)),
                    ui::button(border * 3 + button_width * 2, border * 2 + button_heigt, button_width, button_heigt, "right".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleRight)),
                    ui::button(border *2 + button_width, border *3 + button_heigt * 2, button_width, button_heigt, "down".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleDown)),
                    ui::button(border, border * 4 + button_heigt * 3, (border * 2 + button_width * 3)/2 -10, button_heigt, "     -".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleDecrease)),
                    ui::button(border + (border * 2 + button_width * 3)/2 + 10, border * 4 + button_heigt * 3, (border * 2 + button_width * 3)/2 -10, button_heigt, "     +".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleInlarge)),
                    menu_button,
                    moved_element,
                    change_button,
            ]
        }
        Screen::Color => {
            let bg = Color::from_hex(0x333333);
            vec![
                ui::slider(20, 40, 40, 190, 0, 255, m.color.red as i32, bg, Color::from_hex(0xff0000), |x| Message::ColorRed(x)),
                ui::slider(80, 40, 40, 190, 0, 255, m.color.green as i32, bg, Color::from_hex(0x00ff00), |x| Message::ColorGreen(x)),
                ui::slider(140, 40, 40, 190, 0, 255, m.color.blue as i32, bg, Color::from_hex(0x0000ff), |x| Message::ColorBlue(x)),
                ui::rectangle(300, 100, 50, 50, m.color, true),
                menu_button,
            ]
        }
        Screen::Keyboard => {
            vec![
                ui::keyboard(m.keyboard_text.clone(), Color::from_hex(0xffffff)),
            ]
        }
    }
}

pub fn update(m: Model, msg: Message) -> Model{
    match msg {
        Message::ToMenuScreen => Model{screen: Screen::Menu, ..m},
        Message::ToWidgetsScreen => Model{screen: Screen::Widgets, ..m},
        Message::ToColorScreen => Model{screen: Screen::Color, ..m},
        Message::ToDotScreen => Model{screen: Screen::Dot, ..m},
        Message::ToKeyboardScreen => Model{screen: Screen::Keyboard, ..m},
        Message::CircleDecrease => Model{radius_circle: (m.radius_circle -5).max(1), ..m},
        Message::CircleInlarge => Model{radius_circle: (m.radius_circle + 5).min(60), ..m},
        Message::CircleDown => Model{position_circle_y: (m.position_circle_y + 3).min(300), ..m},
        Message::CircleUp => Model{position_circle_y: (m.position_circle_y - 3).max(50), ..m}, //if circle should also be allowed to have midpoint above screen, type of position_circle_y has to be changed
        Message::CircleLeft => Model{position_circle_x : (m.position_circle_x - 3).max(250), ..m}, //maybe type of position_circle_x has to be changed as above
        Message::CircleRight => Model{position_circle_x : (m.position_circle_x + 3).min(510), ..m},
        Message::CircleRectangle => Model{dot_is_rec: !m.dot_is_rec, ..m},
        Message::ColorRed(x) => Model{color: Color::rgb(x as u8, m.color.green, m.color.blue), ..m},
        Message::ColorGreen(x) => Model{color: Color::rgb(m.color.red, x as u8, m.color.blue), ..m},
        Message::ColorBlue(x) => Model{color: Color::rgb(m.color.red, m.color.green, x as u8), ..m},
        Message::KeyboardButtonMessage(c) => {
            let mut text = m.keyboard_text.clone();
            text.push(c);
            Model{keyboard_text: text, ..m}
        },
    }
}