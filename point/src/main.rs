#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

fn translate(point: &Point, delta: &Point) -> Point {
    Point {
        x: point.x + delta.x,
        y: point.y + delta.y,
        z: point.z + delta.z
    }
}

fn main() {
    let origin = Point {
        x: 3.0,
        y: 0.0,
        z: 2.0,
    };

    let delta = Point {
        x: 1.0,
        y: 0.0,
        z: 1.0,
    };

    let new_point = translate(&origin, &delta); 

    println!("{:?}", new_point);
}
