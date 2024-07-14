fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    println!("{}", args[1]);
    println!("{}", args.len());
    // let answer = "42\n";
    // let answer = answer.trim().parse::<i32>().unwrap();
    // println!("{answer}");

    let x = 7889;
    let y:i8 = 10;
    let z = x + y;
}
