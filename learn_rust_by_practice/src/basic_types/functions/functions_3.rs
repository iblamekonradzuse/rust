fn main(){
    never_return();

    println!("fail");
}

fn never_return() -> ! {
    panic!();
}