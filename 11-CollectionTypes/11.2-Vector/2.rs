fn main() {
    let mut v1 = Vec::from([1, 2, 4]);

    v1.pop();
    v1.push(3);

    println!("{:?}", v1);

    let mut v2 = Vec::new();
    v2.extend(&v1);

    assert_eq!(v1, v2);

    println!("{:?}", v2);
    println!("suc");
}
