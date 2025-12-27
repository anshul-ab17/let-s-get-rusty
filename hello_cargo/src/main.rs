fn main(){
    let ans :u32 =sum(3,4);
    println!("{}",ans);
    println!("heey");

    let name = String::from("ab");
    println!("{}",name);

    let vname: Vec<i32>= vec![1,2,3];
    println!("{:?}",vname);

}

fn sum(a:u32,b:u32) ->u32 {
    return a+b;
}