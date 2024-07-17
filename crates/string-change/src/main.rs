fn main() {
    let text = String::from("Foo");
    println!("Main: {text}");
    display(&text);
    
    let new = change(text);
    //println!("Main: {text}");
    println!("New: {new}");
    //display(&text);
}

fn display(txt: &str) {
    println!("Display: {txt}")
}

fn change(mut txt: String) -> String {
    //*txt = String::from("Bar");
    txt.push_str(" Bar");
    txt
}
