

// fn move() {
//     let x =5;
//     let y= x;

//     println!("{}",x);
//     println!("{}",y);
// }

fn main() {
 
    let x = 5;
    takes_copy(x); 
    let s = String::from("Rusty!"); 
    takes_ownership(s); 
    let s1 = gives_ownership();
    let s2 = String::from("ab");
    let s3 = takes_and_gives_back(s2);
}
fn takes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} 

fn gives_ownership() -> String {
    let some_string = String::from("ab");
    some_string
}
fn takes_and_gives_back(some_string: String)-> String {
    some_string
}
