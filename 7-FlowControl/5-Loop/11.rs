fn main(){
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20{
                println!("count is =>20");
                break 'inner1;
            }
            println!("adding +2");
            count += 2;
        }
        count += 5;
        println!("adding +5");

        'inner2: loop {
            if count >= 30{
                println!("count is 30");
                break 'outer;
            }

            continue 'outer;
        }
    }
    assert!(count == 30);
    println!("suc");
}