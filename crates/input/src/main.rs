fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    println!("{}", args[1]);
    println!("{}", args.len());
    // let answer = "42\n";
    // let answer = answer.trim().parse::<i32>().unwrap();
    // println!("{answer}");

    let x = 78_i8;
    let y = 1000_i16;
    let z = x + y as i8;
}
