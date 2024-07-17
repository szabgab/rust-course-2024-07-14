//#[derive(PartialEq)]
struct Foo {
    name: String,
    num: u32,
}

fn main() {
    let a = Foo {
        name: String::from("xabc", num: 2),
    };
    let b = Foo {
        name: String::from("abc", num: 3),
    };

    match a {
        b => println!("matched"),
        _ => println!("not matched"),
    }

    println!("Hello, world!");
}
