use graphics::model::Model;
use stm32f7::lcd::{Color, Framebuffer, Layer};
use graphics::circle;
use arrayvec::ArrayVec;

pub struct BoundingBox {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl BoundingBox {
    pub fn new(mut min_x: i32, mut min_y: i32, mut max_x: i32, mut max_y: i32) -> BoundingBox {
        if min_x < 0 {
            min_x = 0;
        }
        if min_y < 0 {
            min_y = 0;
        }
        if max_x >= 480 {
            max_x = 479;
        }
        if max_y >= 272 {
            max_y = 271;
        }
        BoundingBox{
            min_x,
            min_y,
            max_x,
            max_y
        }
    }
}

pub struct View{
    bbs1: ArrayVec<[BoundingBox; 3]>,
    bbs1b: ArrayVec<[BoundingBox; 3]>,
    active_layer: bool,
}

impl View {
    pub fn new() -> View {
        View{
            bbs1: ArrayVec::new(),
            bbs1b: ArrayVec::new(),
            active_layer: false,
        }
    }

    pub fn view<T: Framebuffer>(self: &mut View, m: &Model, layer1: &mut Layer<T>, layer1b: &mut Layer<T>) -> bool{

        let layer_inactive = if self.active_layer {
            layer1
        }else{
            layer1b
        };

        let bbs_inactive = if self.active_layer {
            &mut self.bbs1
        }else {
            &mut self.bbs1b
        };

        for bb in bbs_inactive.iter() {
            for y in bb.min_y..=bb.max_y {
                for x in bb.min_x..=bb.max_x {
                    layer_inactive.print_point_color_at(x as usize, y as usize, Color::from_hex(0x0000FF));
                }
            }
        }

        bbs_inactive.clear();

        match m.cursor.first_contact {
            Some(ref p) => bbs_inactive.push(circle::draw_circle(layer_inactive, &p, m.r, Color::from_hex(0xFFFFFF))),
            None => None,
        };

        match m.cursor.second_contact {
            Some(ref p) => bbs_inactive.push(circle::draw_circle(layer_inactive, &p, 10, Color::from_hex(0xFF0000))),
            None => None,
        };

        match m.cursor.last_contact {
            Some(ref p) => bbs_inactive.push(circle::draw_circle(layer_inactive, &p, m.r, Color::from_hex(0x00FF00))),
            None => None,
        };

        self.active_layer = !self.active_layer;

        self.active_layer
    }
}