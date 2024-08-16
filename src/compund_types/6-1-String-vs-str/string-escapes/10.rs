fn main(){
    let raw_str = r"escapes dont work here: \x3F \ u{211D}";

    println!("{}",raw_str);
    let quotes = r#"and then i said: "there is no escape!""#;

    println!("{}", quotes);

    let delimiter = r###"A string with "# in it. And even "##!""###;
    println!("{}",delimiter);
}
