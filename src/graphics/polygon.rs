use stm32f7::lcd::{Layer, Framebuffer, Color};
use graphics::{line, point::Point};

pub fn draw_polygon<T: Framebuffer> (lcd: &mut Layer<T>, points: &[Point], color: Color, fill: bool) {
    if !(points.len() > 2) {
        return;
    }

    if fill {
        fill_polygon(lcd, points, color);
    } else {
        let mut last_point = &points[points.len()-1];
        for point in points {
            line::draw_line(lcd, last_point, point, color);
            last_point = point;
        }
    }
}

fn get_bounds(points: &[Point]) -> (Point, Point) {
    let mut min_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_x = min_x;
    let mut max_y = min_y;

    for p in points {
        if p.x < min_x {
            min_x = p.x;
        }
        if p.y < min_y {
            min_y = p.y;
        }
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }

    (
        Point {x: min_x, y: min_y,},
        Point {x: max_x, y: max_y,},
    )
}

/*
 * Polygon fill algorithm by Darel Rex Finley (originally in C)
 * URL: http://alienryderflex.com/polygon_fill/
 * visited: 12:59:37
 */
fn fill_polygon<T: Framebuffer> (lcd: &mut Layer<T>, points: &[Point], color: Color) {
    let mut node_x = [0 as i32, points.len() as i32];
    let bounds = get_bounds(points);
    let poly_size = points.len();

    // loop through the rows of the image
    for pixel_y in bounds.0.y..bounds.1.y {
        // build a list of nodes
        let mut nodes = 0;
        let mut j = poly_size - 1;
        for i in 0..poly_size {
            let bool_a = points[i].y < pixel_y && points[j].y >= pixel_y;
            let bool_b = points[j].y < pixel_y && points[i].y >= pixel_y;
            if bool_a || bool_b {
                let a = points[i].x as i32;
                let b = pixel_y as i32 - points[i].y as i32;
                let c = points[j].x as i32 - points[i].x as i32;
                let d = points[j].y as i32 - points[i].y as i32;
                node_x[nodes] = a + b * c / d;
                nodes += 1;
            }
            j = i;
        }

        // sort the nodes with bubble sort
        let mut i = 0;
        while i + 1 < nodes {
            if node_x[i] > node_x[i + 1] {
                node_x.swap(i, i + 1);
                if i != 0 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }

        // fill the pixels between node pairs
        for i in (0..nodes).filter(|e| e % 2 == 0) {
            if node_x[i] >= bounds.1.x as i32 {
                break;
            }
            if node_x[i + 1] > bounds.0.x as i32 {
                if node_x[i] < bounds.0.x as i32 {
                    node_x[i] = bounds.0.x as i32;
                }
                if node_x[i + 1] > bounds.1.x as i32 {
                    node_x[i + 1] = bounds.1.x as i32;
                }
                for pixel_x in node_x[i]..node_x[i + 1] {
                    lcd.print_point_color_at(pixel_x as usize, pixel_y, color);
                }
            }
        }
    }
}
