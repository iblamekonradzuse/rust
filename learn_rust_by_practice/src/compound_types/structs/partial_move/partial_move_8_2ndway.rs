#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main(){
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust by Practice".to_string()
    };

    let _name: String = f.name.clone();

    println!("{}, {}, {:?}", _name, f.data, f);
}