fn main() {
    let mut text = String::from("hello");
    println!("{text}");

    let other = &text;
    println!("{other}");

    // text = other;
    println!("{text}");

}
