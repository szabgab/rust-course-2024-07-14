fn main() {
    let mut text = String::new();

    println!("{} {} {:p} {:?}", text.len(), text.capacity(), &text, text.as_ptr());
    text.push('a');
    println!("{} {} {:p} {:?}", text.len(), text.capacity(), &text, text.as_ptr());
    text.push('😁');
    println!("{} {} {:p} {:?}", text.len(), text.capacity(), &text, text.as_ptr());
}