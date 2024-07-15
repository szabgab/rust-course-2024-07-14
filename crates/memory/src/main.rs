fn main() {
    let mut text = String::new();

    println!("{} {} {:p} {:?}", text.len(), text.capacity(), &text, text.as_ptr());
    text.push('a');
    println!("{} {} {:p} {:?}", text.len(), text.capacity(), &text, text.as_ptr());
    text.push('😁');
    println!("{} {} {:p} {:?}", text.len(), text.capacity(), &text, text.as_ptr());

    let hi = String::from("hi");
    println!("{} {} {:p} {:?}", text.len(), text.capacity(), &hi, hi.as_ptr());

    text.push_str("qqrq");
    println!("{} {} {:p} {:?}", text.len(), text.capacity(), &text, text.as_ptr());

    text.push_str("123456789012345678kjhdsfksahfka");
    println!("{} {} {:p} {:?}", text.len(), text.capacity(), &text, text.as_ptr());

}