fn main(){
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y: (i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x ,y );
}
//&str is a string which is hard coded to binary itself instead of getting kept in memory 
//it is immutable 