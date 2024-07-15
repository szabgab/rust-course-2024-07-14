fn main() {
    let text = String::from("Foo Bar\n");
    println!("{text}");

    //let new_name = name.trim_end().to_owned();
    println!("{:p}", &text);

    let other = &text;
    println!("{other}");

    // text = other;
    println!("{text}");

}
