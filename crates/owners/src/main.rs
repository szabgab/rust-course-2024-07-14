fn main() {
    let text = "Foo Bar";
    

    let text = String::from("Foo Bar\n");
    println!("{text}");

    // ptr  
    // len
    // capacity
    println!("{:p} {:?} {}", &text, text.as_ptr(), text.len());

    let short = text.trim_end();
    println!("{:p} {:?} {}", &short, short.as_ptr(), short.len());
    println!("{:p} {:?} {}", &text, text.as_ptr(), text.len());
    
    // let other = &text;
    // println!("{other}");

    // // text = other;
    // println!("{text}");

}
