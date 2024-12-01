use std::fs::File;
use std::io::{self, Read};

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?; // Open the file
    let mut contents = String::new(); // Create a string to store the contents
    file.read_to_string(&mut contents)?; // Read the file's content into the string
    Ok(contents) // Return the content
}

use aoc_rust::day1::{part1, part2};

fn main() {
    let inp = read_file("input/2024/day1.txt");
    match inp {
        Ok(val) => {
            let p1 = part1(&val);
            let p2 = part2(&val);
            println!("p1: {}", p1);
            println!("p2: {}", p2);
        }
        Err(e) => println!("error: {}", e),
    }
}
