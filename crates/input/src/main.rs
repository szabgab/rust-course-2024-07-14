fn main() {
    let args = std::env::args().collect();
    let answer = "42\n";
    let answer = answer.trim().parse::<i32>().unwrap();
    println!("{answer}");
}
