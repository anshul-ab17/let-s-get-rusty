use std::io;
use circle::Circle;
pub mod circle;

fn main() {
    let mut input = String::new();
    println!("enter the radius:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read i/p");

    let radius: f32 = input.trim().parse().expect("enter the radius");

    let circle = Circle { radius };

    println!("Area of the circle is: {}", circle.area());
    println!("Perimeter of the circle is: {}", circle.perimeter());

    Circle::static_circle();
}