use std::fmt::{write,Debug, Formatter};


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
    let user =User{
        username:String::from("ab")
    };
    println!("{:?}",user)

}