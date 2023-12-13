fn main() {
    let file_path: String = "input.txt".to_string();
    println!("playground");
    println!("In file {}", file_path);

    println!(
        "{}",
        include_str!("../input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap(),
    );
}
