use std::{env, fs};


fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{}", args[1]);

    let paths = fs::read_dir(args[1].clone()).expect("Error");
    for path in paths {
        println!("{:?}", path.unwrap().path().to_str());
    };
}
