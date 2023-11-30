use std::env;
use std::fs;

fn main() {
    let file_path: String = "input.txt".to_string();
    println!("playground");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!(
        "{}",
        //contents
        include_str!("../input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap(),
    );
}
