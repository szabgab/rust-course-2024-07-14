fn main() {
    // let text = "Foo Bar";
    // // let other = &text[0..3];
    // let other = &text[4..];
    // println!("{other}");

    let text = String::from("Foo Bar\n");
    println!("{text}");
    let fname = &text[0..3];
    println!("{fname}");


    // ptr  
    // len
    // capacity
    println!("{:p} {:?} {}", &text, text.as_ptr(), text.len());

    // let short = text.trim_end();
    // println!("{:p} {:?} {}", &short, short.as_ptr(), short.len());
    // println!("{:p} {:?} {}", &text, text.as_ptr(), text.len());

    let short = text.trim_end().to_owned();
    println!("{:p} {:?} {}", &short, short.as_ptr(), short.len());
    println!("{:p} {:?} {}", &text, text.as_ptr(), text.len());
    
    // let other = &text;
    // println!("{other}");

    // // text = other;
    // println!("{text}");

}
