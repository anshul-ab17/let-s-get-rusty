// #![deny(dead_code)] or #[warn(unused)]) or // #[warn(dead_code)] 

// [allow(unused)
// #![allow(dead_code)] applies to the ENTIRE crate (file)

#[allow(dead_code)]  //applies to ONE item
use direction::direction::Direction;
use direction::shape::Shape;

//folder_name :: file_name:: enum /struct

fn main() {
    let way= Direction::East;
    steer(way);

    let s1 =Shape::Square(10);
    let s2 =Shape::Circle(10);
    let s3 =Shape::Rectangle(10,5);

    println!("{:?},{:?},{:?}", s1,s2,s3);
}

fn steer(dir:Direction){
    match dir{
        Direction::North=> println!("from N"),
        Direction::South=> println!("from S"),
        _=> println!("form E|W")
    }

}