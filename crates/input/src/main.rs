fn main() {
    let answer = "42\n";
    let answer = answer.trim().parse::<i32>().unwrap();
    println!("{answer}");
}
