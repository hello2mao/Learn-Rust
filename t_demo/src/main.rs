
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let point: Point<i32> = Point {
        x: 1,
        y: 2,
    };
    println!("point = {:#?}", point);
    println!("x = {}", point.get_x());

    println!("Hello, world!");
}
