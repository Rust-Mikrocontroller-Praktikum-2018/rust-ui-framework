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

use graphics::{Message, TouchEvent};
use alloc::Vec;
use alloc::boxed::Box;
use stm32f7::lcd::{FramebufferAl88, FramebufferArgb8888};
//use stm32f7::lcd::Color;
mod graphics;

use core::any::Any;

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
                counter: i32,
                c2: i32,
            };

            // enum Message

            let mut model = Model{counter: 0, c2: 0};

            fn view(m: &Model) -> Vec<Box<UIComponent>> {
                vec![Box::new(graphics::button::Button::new(10, 50, 100, 30, "Inc", Some(Message::Increment))),
                     Box::new(graphics::button::Button::new(10, 100, 100, 30, "Dec", Some(Message::Decrement))),
                     Box::new(graphics::button::Button::new(150+10*m.counter as usize, 75+10*m.c2 as usize, 100, 30, "!!!", None))]
            }

            fn update(m: Model, msg: Message) -> Model{
                match msg {
                    Message::Increment => Model{counter: m.counter+1, c2: m.c2},
                    Message::Decrement => Model{counter: m.counter, c2: m.c2+1},
                }
            }
            // -------------------------------------------------------------------------------------


            // -------------------------------------------------------------------------------------
            // ---  Framework code -----------------------------------------------------------------
            // -------------------------------------------------------------------------------------

            // enum TouchEvent

            use graphics::point::Point;
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
                    (Some(idx), Some(evt)) => widgets[idx].on_touch(&evt),
                    _ => None,
                };

                // process message
                if new_msg.is_some(){
                    let msg = new_msg.unwrap();

                    model = update(model, msg);
                    let new_widgets = view(&model);
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
    for (idx, w) in widgets.iter().enumerate(){
        let old_widget = if idx < old_widgets.len() {
            Some(old_widgets[idx].as_any() as &Any)
        }else{
            None
        };
        w.draw(old_widget, lcd_ui, lcd_text);
    }
}