fn main() {
    let a = 7;
    let b = 8;
    println!("{}", add(a, b));

    let c = 7_i8;
    let d = 29_i8;
    println!("{}", add(c, d));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
