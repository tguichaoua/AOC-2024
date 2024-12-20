mod circle;

pub use circle::Circle;

pub fn circle(radius: i32) -> Circle {
    Circle::new(radius)
}
