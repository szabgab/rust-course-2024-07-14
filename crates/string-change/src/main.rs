fn main() {
    let mut text = String::from("Foo");
    println!("Main: {text}");
    display(&text);
    let new = change(text);
    println!("Main: {text}");
    display(&text);
}

fn display(txt: &str) {
    println!("Display: {txt}")
}

fn change(mut txt: String) {
    //*txt = String::from("Bar");
    txt.push_str(" Bar");
}
