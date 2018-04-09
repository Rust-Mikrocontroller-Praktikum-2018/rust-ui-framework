use graphics::point::Point;

pub struct Model {
    pub p: Point,
    pub r: i32,
    pub cursor: CursorModel,
}

pub struct CursorModel {
    pub first_contact: Option<Point>,
    pub second_contact: Option<Point>,
    pub last_contact: Option<Point>,
}