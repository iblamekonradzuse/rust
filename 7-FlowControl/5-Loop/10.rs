fn main(){
    let mut counter: i32 = 0;

    let result: i32 = loop{
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
    println!("suc");
    println!("counter is={}",counter);
    println!("result is={}",result);
}