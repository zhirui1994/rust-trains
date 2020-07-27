use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Search for {}", query);
    println!("In file {}", filename);

    let mut contents = String::new();
    File::open(filename)
        .expect("File not found!")
        .read_to_string(&mut contents)
        .expect(
            &format!("Something went wrong reading the file {}", filename)
        );
    
    println!("With text:\n{}", contents);

}
