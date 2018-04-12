#![feature(lang_items)]
#![feature(const_fn)]
#![feature(alloc)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![no_std]
#![no_main]

//#[macro_use]
extern crate stm32f7_discovery as stm32f7;

// initialization routines for .data and .bss

#[macro_use]
extern crate alloc;
extern crate arrayvec;
extern crate compiler_builtins;
extern crate r0;
extern crate smoltcp;

// hardware register structs with accessor methods
use stm32f7::{audio, board, embedded, lcd, sdram, system_clock, touch, i2c};
use graphics::ui_component::UIComponent;

use graphics::point::Point;
use graphics::{Message, TouchEvent, Screen};
use alloc::Vec;
use alloc::boxed::Box;
use stm32f7::lcd::{FramebufferAl88, FramebufferArgb8888};
use stm32f7::lcd::Color;
mod graphics;
use alloc::string::ToString;

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static mut __DATA_END: u32;
        static mut __DATA_START: u32;

        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }

    // initializes the .data section (copy the data segment initializers from flash to RAM)
    r0::init_data(&mut __DATA_START, &mut __DATA_END, &__DATA_LOAD);
    // zeroes the .bss section
    r0::zero_bss(&mut __BSS_START, &__BSS_END);

    stm32f7::heap::init();

    // enable floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);
    asm!("DSB; ISB;"::::"volatile"); // pipeline flush

    main(board::hw());
}

// WORKAROUND: rust compiler will inline & reorder fp instructions into
#[inline(never)] //             reset() before the FPU is initialized
fn main(hw: board::Hardware) -> ! {
    use embedded::interfaces::gpio::{self, Gpio};

    let x = vec![1, 2, 3, 4, 5];
    assert_eq!(x.len(), 5);
    assert_eq!(x[3], 4);

    let board::Hardware {
        rcc,
        pwr,
        flash,
        fmc,
        ltdc,
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
        i2c_3,
        sai_2,
        syscfg,
        nvic,
        exti,
        ..
    } = hw;

    let mut gpio = Gpio::new(
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
    );

    system_clock::init(rcc, pwr, flash);

    // enable all gpio ports
    rcc.ahb1enr.update(|r| {
        r.set_gpioaen(true);
        r.set_gpioben(true);
        r.set_gpiocen(true);
        r.set_gpioden(true);
        r.set_gpioeen(true);
        r.set_gpiofen(true);
        r.set_gpiogen(true);
        r.set_gpiohen(true);
        r.set_gpioien(true);
        r.set_gpiojen(true);
        r.set_gpioken(true);
    });

    // configure led pin as output pin
    let led_pin = (gpio::Port::PortI, gpio::Pin::Pin1);
    let mut led = gpio.to_output(
        led_pin,
        gpio::OutputType::PushPull,
        gpio::OutputSpeed::Low,
        gpio::Resistor::NoPull,
    ).expect("led pin already in use");

    // turn led on
    led.set(true);

    let button_pin = (gpio::Port::PortI, gpio::Pin::Pin11);
    let _ = gpio.to_input(button_pin, gpio::Resistor::NoPull)
        .expect("button pin already in use");

    // init sdram (needed for display buffer)
    sdram::init(rcc, fmc, &mut gpio);

    // lcd controller
    let mut lcd = lcd::init(ltdc, rcc, &mut gpio);
    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();
    let mut layer_1b = lcd.layer_1b().unwrap();

    layer_1.clear();
    layer_2.clear();
    layer_1b.clear();
    lcd.set_background_color(lcd::Color::from_hex(0x000000));
    //lcd::init_stdout(layer_2);

    // i2c
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);
    i2c_3.test_1();
    i2c_3.test_2();

    // sai and stereo microphone
    audio::init_sai_2_pins(&mut gpio);
    audio::init_sai_2(sai_2, rcc);
    assert!(audio::init_wm8994(&mut i2c_3).is_ok());

    touch::check_family_id(&mut i2c_3).unwrap();

    //let mut audio_writer = layer_1.audio_writer();
    // let mut text_writer = layer_2.text_writer();
    //let mut last_led_toggle = system_clock::ticks();

    use stm32f7::board::embedded::components::gpio::stm32f7::Pin;
    use stm32f7::board::embedded::interfaces::gpio::Port;
    use stm32f7::exti::{EdgeDetection, Exti, ExtiLine};

    let mut exti = Exti::new(exti);
    let mut exti_handle = exti.register(
        ExtiLine::Gpio(Port::PortI, Pin::Pin11),
        EdgeDetection::FallingEdge,
        syscfg,
    ).unwrap();

    use stm32f7::interrupts::interrupt_request::InterruptRequest;
    use stm32f7::interrupts::{scope, Priority};

    scope(
        nvic,
        |_| {},
        |interrupt_table| {
            let _ =
                interrupt_table.register(InterruptRequest::Exti10to15, Priority::P1, move || {
                    exti_handle.clear_pending_state();
                    // choose a new background color
                    //let new_color =
                    //    ((system_clock::ticks() as u32).wrapping_mul(19801)) % 0x1000000;
                    //lcd.set_background_color(lcd::Color::from_hex(new_color));
                });


            // -------------------------------------------------------------------------------------
            // ---  User code ----------------------------------------------------------------------
            // -------------------------------------------------------------------------------------
            struct Model {
                screen: Screen,
                position_circle_x: usize,
                position_circle_y: usize,
                radius_circle: i32,
                color: Color,
                dot_is_rec: bool,
                /* counter: i32,
                c2: i32,
                show_text: bool,
                slider_value: i32, */
            };

            // enum Message

            let mut model = Model{screen: Screen::Menu, position_circle_x: 360, position_circle_y: 135, radius_circle: 30, color: Color::rgb(255, 0, 100), dot_is_rec: false};

            // let _slider_message = |x| Message::OnChange(x);

            use graphics::ui;

            fn view(m: &Model) -> Vec<Box<UIComponent>> {
                let menu_button = ui::button(430, 20, 30, 30, " X".to_string(), Color::rgb(100, 100, 100), Some(Message::ToMenuScreen));

                match m.screen {
                    Screen::Menu => {
                        vec![
                            ui::button(20, 20, 150, 30, "Widgets Demo".to_string(), Color::rgb(100, 100, 100), Some(Message::ToWidgetsScreen)),
                            ui::button(20, 60, 150, 30, "Color Picker Demo".to_string(), Color::rgb(100, 100, 100), Some(Message::ToColorScreen)),
                            ui::button(20, 100, 150, 30, "Dot Demo".to_string(), Color::rgb(100, 100, 100), Some(Message::ToDotScreen)),
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
                             ui::slider(430, 100, 30, 150, 0, 100, 75, Color::from_hex(0x333333), Color::from_hex(0xffffff), |x| Message::OnChange(x)),
                        ]
                    }
                    Screen::Dot => {
                        let moved_element :Box<UIComponent> = if !m.dot_is_rec {
                            ui::circle(m.position_circle_x, m.position_circle_y, m.radius_circle, m.color, true)
                        } else {
                            ui::rectangle(m.position_circle_x - m.radius_circle as usize, m.position_circle_y - m.radius_circle as usize, (m.radius_circle * 2) as usize, (m.radius_circle * 2) as usize, m.color, true)
                        };
                        let change_button = if !m.dot_is_rec {
                            ui::button(35, 200, 70, 30, "circle to rectangle".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleRectangle))
                        } else {
                            ui::button(35, 200, 70, 30, "rectangle to circle".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleRectangle))
                        };
                        vec![ui::button(20, 60, 20, 20, "left".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleLeft)),
                             ui::button(60, 20, 20, 20, "up".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleUp)),
                             ui::button(100, 60, 20, 20, "right".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleRight)),
                             ui::button(60, 100, 20, 20, "down".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleDown)),
                             ui::button(20, 140, 50, 20, "-".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleDecrease)),
                             ui::button(70, 140, 50, 20, "+".to_string(), Color::rgb(100, 100, 100), Some(Message::CircleInlarge)),
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
                    _ => vec![menu_button]
                }
                // let w_new : Box<UIComponent> = if m.show_text{
                //     Box::new(graphics::button::Button::new(150+10*m.counter as usize, 75+10*m.c2 as usize, 100, 30, m.counter.to_string(), Color::rgb((m.counter*20) as u8, (m.counter*20) as u8, (m.counter*20) as u8), None))
                // }else{
                //     Box::new(graphics::rectangle::Rectangle::new(130, 10, 50, 50, Color::from_hex(0xff0000), true))
                // };
                // vec![
                //     Box::new(graphics::button::Button::new(10, 50, 100, 30, "Inc".to_string(), Color::rgb(0, 0, 0), Some(Message::Increment))),
                //     Box::new(graphics::button::Button::new(10, 100, 100, 30, "Dec".to_string(), Color::rgb(0, 0, 0), Some(Message::Decrement))),
                //     w_new,
                //     Box::new(graphics::slider::Slider::new(400, 30, 20, 150, 0, 1000, m.slider_value, Color::rgb(100, 100, 100), Color::rgb(255, 80, 80), |x|{Message::OnChange(x)})),
                //     Box::new(graphics::button::Button::new(450, 30, 20, 20, m.slider_value.to_string(), Color::rgb(200, 0, 0), None)),
                //     Box::new(graphics::polygon::Polygon::new(vec![Point{x: 200, y: (m.c2*10+20) as usize}, Point{x: 150, y: 120}, Point{x: 170, y: 200}], Color::from_hex(0xffff00), true)),
                // ]
            }

            fn update(m: Model, msg: Message) -> Model{
                match msg {
                    Message::ToMenuScreen => Model{screen: Screen::Menu, ..m},
                    Message::ToWidgetsScreen => Model{screen: Screen::Widgets, ..m},
                    Message::ToColorScreen => Model{screen: Screen::Color, ..m},
                    Message::ToDotScreen => Model{screen: Screen::Dot, ..m},
                    Message::CircleDecrease => Model{radius_circle: (m.radius_circle -5).min(1), ..m},
                    Message::CircleInlarge => Model{radius_circle: (m.radius_circle + 5).max(60), ..m},
                    Message::CircleDown => Model{position_circle_y: (m.position_circle_y + 3).max(300), ..m},
                    Message::CircleUp => Model{position_circle_y: (m.position_circle_y - 3).min(50), ..m}, //if circle should also be allowed to have midpoint above screen, type of position_circle_y has to be changed
                    Message::CircleLeft => Model{position_circle_x : (m.position_circle_x - 3).min(250), ..m}, //maybe type of position_circle_x has to be changed as above
                    Message::CircleRight => Model{position_circle_x : (m.position_circle_x + 3).max(510), ..m},
                    Message::CircleRectangle => Model{dot_is_rec: !m.dot_is_rec, ..m},
                    Message::ColorRed(x) => Model{color: Color::rgb(x as u8, m.color.green, m.color.blue), ..m},
                    Message::ColorGreen(x) => Model{color: Color::rgb(m.color.red, x as u8, m.color.blue), ..m},
                    Message::ColorBlue(x) => Model{color: Color::rgb(m.color.red, m.color.green, x as u8), ..m},
                    // Message::Increment => Model{counter: m.counter+1, ..m},
                    // Message::Decrement => Model{c2: m.c2+1, ..m},
                    // Message::OnChange(x) => Model{slider_value: x, ..m},
                    _ => m
                }
            }
            // -------------------------------------------------------------------------------------


            // -------------------------------------------------------------------------------------
            // ---  Framework code -----------------------------------------------------------------
            // -------------------------------------------------------------------------------------

            // enum TouchEvent

            let mut active_widget: Option<usize> = None;
            let mut widgets: Vec<Box<UIComponent>> = view(&model);

            // initial draw
            draw(&widgets, &vec![], &mut layer_1, &mut layer_2);

            let mut prev_touch: Option<Point> = None;

            loop{
                // get touch event
                let touches = touch::touches(&mut i2c_3).unwrap();
                let curr_touch = if touches.len() == 1 {
                    Some(Point{x: touches[0].x as usize, y: touches[0].y as usize})
                }else{
                    None
                };
                let touch_event = match (prev_touch, curr_touch) {
                    (None, Some(p)) => Some(TouchEvent::Pressed(p)),
                    (Some(_), Some(p)) => Some(TouchEvent::Moved(p)),
                    (Some(_), None) => Some(TouchEvent::Released),
                    (None, None) => None,
                };
                prev_touch = curr_touch;

                // continue if no touch event
                if touch_event.is_none() {continue;}

                // update active_widget if touch_event is "Pressed"
                active_widget = match touch_event {
                    Some(TouchEvent::Pressed(p)) => find_widget(&p, &widgets),
                    _ => active_widget,
                };

                // send touch event to active widget
                let new_msg = match (active_widget, &touch_event) {
                    (Some(idx), &Some(ref evt)) => widgets[idx].on_touch(&evt),
                    _ => None,
                };

                // process message
                if new_msg.is_some(){
                    let msg = new_msg.unwrap();

                    let prev_screen = model.screen;
                    model = update(model, msg);
                    let new_widgets = view(&model);

                    // clear all if screen change
                    if prev_screen != model.screen {
                        for w in widgets{
                            w.clear(&mut layer_1, &mut layer_2);
                        }
                        widgets = vec![];
                    }

                    draw(&new_widgets, &widgets, &mut layer_1, &mut layer_2);
                    widgets = new_widgets;
                }

                // reset active widget if touch event is "Released"
                match touch_event {
                    Some(TouchEvent::Released) => active_widget = None,
                    _ => (),
                }
            }
            // -------------------------------------------------------------------------------------
        },
    )
}

fn find_widget(p: &graphics::point::Point, widgets: &Vec<Box<UIComponent>>) -> Option<usize>{
    for (idx, w) in widgets.iter().enumerate(){
        if w.is_in_bounding_box(p){
            return Some(idx);
        }
    }
    return None;
}

use stm32f7::lcd::Layer;
fn draw(widgets: &Vec<Box<UIComponent>>, old_widgets: &Vec<Box<UIComponent>>, lcd_ui: &mut Layer<FramebufferArgb8888>, lcd_text: &mut Layer<FramebufferAl88>){
    for idx in widgets.len()..old_widgets.len(){
        old_widgets[idx].clear(lcd_ui, lcd_text);
    }

    for (idx, w) in widgets.iter().enumerate(){
        let old_widget = if idx < old_widgets.len() {
            Some(old_widgets[idx].as_ref())
        }else{
            None
        };
        w.draw(old_widget, lcd_ui, lcd_text);
    }
}