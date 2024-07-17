macro_rules! prt {
    ($var:expr) => {
        println!(
            "{:2} {:2} {:p} {:15?} '{}'",
            $var.len(),
            $var.capacity(),
            &$var,
            $var.as_ptr(),
            $var
        );
    };
}

fn main() {
    let text = String::from("Foo");
    println!("Main: {text}");
    display(&text);
    prt!(text);

    let new = change(text);
    //println!("Main: {text}");
    println!("New: {new}");
    prt!(new);
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
