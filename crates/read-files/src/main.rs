fn main() {
    let args = std::env::args().collect::<Vec<_>>();    
    println!("{args:?}");

    for filename in &args[1..] {
        println!("{filename}");
        let content = std::fs::read_to_string(filename).unwrap();
        println!("{:?}", content);
    }
}
