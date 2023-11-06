use std::{env, fs};



fn find_file(path: String, file: String) {
    let mut queue = Vec::new();

    queue.push(path.clone());


    while !queue.is_empty() {

        let path = queue.remove(0);

        let paths =  fs::read_dir(path).expect("Error");

        for p in paths {
            let p_unwrapped = p.unwrap();
            if p_unwrapped.path().is_file() && p_unwrapped.file_name().to_str().expect("") == file {
                println!("Found full path: {:?}", p_unwrapped.path().to_str());
                return;
            }
            else if !p_unwrapped.path().is_file() {
                queue.push(p_unwrapped.path().display().to_string());
            }
        }
    }


    println!("File not found");

}


fn recursive_output(p: &str) {
    let paths = fs::read_dir(p).expect("Error");
    for path in paths {

        let path_unwrapped = path.unwrap();
        if path_unwrapped.path().is_file() {
            println!("{:?}", path_unwrapped.path().to_str());
        }
        else {
            recursive_output(path_unwrapped.path().to_str().expect("Error"));
        }
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{}", args[1]);

    if args.len() == 4 && args[2] == "--find" {
        find_file(args[1].clone(), args[3].clone());
    }

    else {
        recursive_output(args[1].as_str());
    }
}
