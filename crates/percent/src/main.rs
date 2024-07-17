struct Foo {
    name: String,
}

fn main() {
    let a = Foo {
        name: String::from("abc"),
    };
    let b = Foo {
        name: String::from("abc"),
    };

    match a {
        b => println!("matched"),
        _ => println!("not matched"),
    }

    println!("Hello, world!");
}
