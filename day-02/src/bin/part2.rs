use std::cmp;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}
const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn calculate_sets(input: &str) -> u32 {
    let sets: &str = &input.replace(";", ",");
    let set_split: Vec<&str> = sets.split(',').collect();
    let mut min_RED: u32 = 0;
    let mut min_GREEN: u32 = 0;
    let mut min_BLUE: u32 = 0;
    for set in set_split {
        let color_split: Vec<&str> = set.trim().split(' ').collect();
        let num: u32 = color_split[0].parse().unwrap();

        match color_split[1] {
            "red" => {
                min_RED = cmp::max(num, min_RED);
            }
            "green" => {
                min_GREEN = cmp::max(num, min_GREEN);
            }
            "blue" => {
                min_BLUE = cmp::max(num, min_BLUE);
            }
            _ => {
                panic!("unknown color!");
            }
        }
    }
    min_RED * min_GREEN * min_BLUE
}

fn part2(input: &str) -> String {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line: &str| {
            let line_str = line.to_string();
            let split: Vec<&str> = line_str.split(':').collect();

            let game_index_split: Vec<&str> = split[0].split(' ').collect();
            let mut game_num: u32 = game_index_split[1].parse().unwrap();
            let mut total: u32 = 0;

            calculate_sets(split[1])
        })
        .collect();

    //.collect();
    let result: u32 = numbers.into_iter().sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part2(test_case);
        assert_eq!(result, "2286".to_string());
    }
}
