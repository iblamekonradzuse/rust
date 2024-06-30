struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main(){
    let u1: User = User {
        email: String::from("lol@gmail.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };
    
    let u2 = set_email(u1);
    println!("suc");
}

fn set_email(u: User) -> User {
    User{
        email: String::from("test@gmail.com"),
        ..u
    }
}