fn main() {
    let text = "שלום עולם";
    println!("{}", text.chars().nth(1).unwrap());
    println!("{}", &text[0..=3]);
    for ch in text.chars() {
        println!("{}", ch);
    }
    let chars = text.chars().collect::<Vec<_>>();
    println!("{:?}", chars);
    println!("{:?}", &chars[1..4]);

    // let mut text = String::from("The black cat climbed the green tree!");
    // let x = &text[0..3];
    // println!("{}", x);
    // let y = text.replace("The", "qqq");

    // println!("{}", text);
    // println!("{}", y);
    // println!("'{}'", &text[4..4]);
    // println!("'{}'", &text[4..=4]);
    // println!("'{}'", &text[4..5]);
    // println!("'{}'", &text[4..9]);
    // println!("'{}'", &text[4..=8]);
    // println!("'{}'", &text[4..]);
    // println!("'{}'", &text[4..text.len()]);
    // println!("'{}'", &text[4..text.len()-1]);
    // println!("'{}'", &text[..4]);

    // println!("'{}'", &text[4..40]);  // thread 'main' panicked at 'byte index 40 is out of bounds
}
