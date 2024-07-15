fn main() {
    let text = String::from("Foo Bar\n");
    println!("{text}");

    println!("{:p} {:?} {}", &text, text.as_ptr(), text.len());

    let short = text.trim_end().to_owned();
    println!("{:p} {:?} {}", &text, text.as_ptr(), text.len());
    
    // let other = &text;
    // println!("{other}");

    // // text = other;
    // println!("{text}");

}
