fn main() {
    let args: Vec<String> = std::env::args().collect();
    dbg!(args);
    println!("{}", &args[1]);
    // let answer = "42\n";
    // let answer = answer.trim().parse::<i32>().unwrap();
    // println!("{answer}");
}
