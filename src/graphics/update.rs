use graphics::model::{Model, CursorModel};
use graphics::point::Point;
use arrayvec::ArrayVec;
use touch::Touch;

pub fn update(m: Model, touches: &ArrayVec<[Touch; 5]>) -> Model {
    let mut current_touch = if touches.len() > 0 {
        Some(Point {
            x: touches[0].x as usize,
            y: touches[0].y as usize,
        })
    } else {
        None
    };

    let (new_first_contact,new_second_contact, new_last_contact) =
    match (m.cursor.first_contact, m.cursor.second_contact, current_touch){
        (Some(p), None, None) => (None, None, Some(p)),
        (Some(p), Some(q), None) => (None, None, Some(q)),
        (None, None, None) => (None, None, m.cursor.last_contact),
        (None, Some(p), None) => (None, None, None), //this case shouldn't occur
        (Some(p), None, Some(q)) => (Some(p), Some(q), None),
        (Some(p), Some(q), Some(r)) => (Some(p), Some(r), None),
        (None, None, Some(p)) => (Some(p), Some(p), None),
        (None, Some(p), Some(q)) => (None, None, None), //this case shouldn't occur
    };

    Model{
        cursor: CursorModel{first_contact: new_first_contact, second_contact: new_second_contact, last_contact: new_last_contact, ..m.cursor},
        ..m
    }
}
