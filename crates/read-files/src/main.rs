fn main() {
    let args = std::env::args().collect::<Vec<_>>();    
    println!("{args:?}");

    fun_name(args);
}

fn fun_name(args: Vec<String>) {
    for filename in &args[1..] {
        println!("{filename}");
        // let content = std::fs::read_to_string(filename).unwrap();
        // let content = std::fs::read_to_string(filename).expect("blblbla");
        // println!("{:?}", content);
        // match std::fs::read_to_string(filename) {
        //     Ok(val) => println!("{val}"),
        //     Err(err) => {
        //         eprintln!("{err}");
        //         //std::process::exit(1);
        //         return;
        //     }
        // }


        // match std::fs::read_to_string(filename) {
        //     Ok(val) => {
        //         println!("{val}");
        //         // ...
        //     },
        //     Err(err) => {
        //         eprintln!("{err}");
        //     }
        // };

        // let content = match std::fs::read_to_string(filename) {
        //     Ok(val) => val,
        //     Err(err) => {
        //         eprintln!("{err}");
        //         String::new()
        //     }
        // };
        // println!("{content}");

        // if let Ok(content) = std::fs::read_to_string(filename) {
        //     println!("{content}");
        // } else {
        //     eprintln!("error");
        // }

        // let content = std::fs::read_to_string(filename).unwrap_or_default();
        // println!("{content}");

        // let content = std::fs::read_to_string(filename).unwrap_or(String::from("qqrq"));
        // println!("{content}");

        // let err = std::fs::read_to_string(filename).unwrap_err();
        // println!("{err}");

        let content = std::fs::read_to_string(filename).unwrap_or_else(op);
        println!("{content}");

    }
}
