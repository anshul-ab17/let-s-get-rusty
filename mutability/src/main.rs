

fn main(){
    let mut str= String::from("ab");
    // mutable and immutable references.
    let a = &mut str;
    a.push_str("17");
    println!("{}",a);
    
    let b = &str;
    let c = &str;
    let d = &str;

    println!("{:?},{:?},{:?}", b,c,d);
}