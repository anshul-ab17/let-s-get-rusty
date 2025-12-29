use std::fmt::{write,Debug, Formattter};


struct User{
    username:string,
    password:string
}
impl Debug for User{
    fn fmt(&self, f:&mut Formattter<'_>)-> std::fmt::Result{
        write!(f,"user: {}", self.username)
    }
}

fn main(){
    let user :User{
        username:String::from("ab");
    };
    println!("{:?}",user)

}