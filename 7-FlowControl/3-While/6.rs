fn main(){
    let mut n: i32 = 1;

    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}",n)
        }
        n += 1;
    }
    println!("n reached {}, so loop is over", n);
}