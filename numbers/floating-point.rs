
fn main() {
    let x: f64 = 1_000.000_1;
    let _y: f32 = 0.12;
    let _z: f64 = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());
    println!("suc");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
