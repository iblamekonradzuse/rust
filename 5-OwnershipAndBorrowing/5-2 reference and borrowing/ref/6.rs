fn main(){
    let c: char = '中';

    let r1: &char = &c;
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    assert_eq!(get_addr(r1),get_addr(r2));

    println!("suc");
}

fn get_addr(r: &char) -> String {
    format!("{:p}",r)
}