use serde::{Serialize,Deserialize} ;

#[derive(Serialize,Deserialize,Debug)]
struct User{
    username:String,
    password:String
}

fn main() {
    let a = User{
        username:String::from(""),
        password:String::from("")
    };

    let str_serialize =serde_json::to_string(&a);
    let json = match str_serialize {
        Ok(str) => {println!("{:?}", str); str}
        Err(_) => { println!("Error occur"); return;}
    };

    let str_deserialized: Result<User, serde_json::Error> =serde_json::from_str(&json);
    match str_deserialized {
        Ok(a) => println!("Deserialized User: {:?}", a),
        Err(e) => println!("Error: {}", e),
    }

}
