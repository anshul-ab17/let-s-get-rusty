use std::fmt::{ Debug };

#[derive(Debug)]
//debug - macro
struct User{
    username:String
}

// debug - here it's a trait
// impl Debug for User{
//     fn fmt(&self, f:&mut Formatter<'_>)-> std::fmt::Result{
//         write!(f,"user: {}", self.username)
//     }
// }

fn main(){
    let username =User{
        username:String::from("ab") 
    };
    println!("{:?}",username)

}