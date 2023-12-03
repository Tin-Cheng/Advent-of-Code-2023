fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}
const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn check_sets(input: &str) -> bool {
    let set_split: Vec<&str> = input.split(',').collect();
    let mut ret: bool = true;
    for set in set_split {
        let color_split: Vec<&str> = set.trim().split(' ').collect();
        let num: u32 = color_split[0].parse().unwrap();

        match color_split[1] {
            "red" => {
                if num > RED {
                    ret = false;
                    break;
                }
            }
            "green" => {
                if num > GREEN {
                    ret = false;
                    break;
                }
            }
            "blue" => {
                if num > BLUE {
                    ret = false;
                    break;
                }
            }
            _ => {
                panic!("unknown color!");
            }
        }
    }
    ret
}

fn part1(input: &str) -> String {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line: &str| {
            let line_str = line.to_string();
            let split: Vec<&str> = line_str.split(':').collect();

            let game_index_split: Vec<&str> = split[0].split(' ').collect();
            let mut game_num: u32 = game_index_split[1].parse().unwrap();

            let sets_split: Vec<&str> = split[1].split(';').collect();
            for sets in sets_split {
                if !check_sets(sets) {
                    game_num = 0;
                    break;
                }
            }
            game_num
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
        let result = part1(test_case);
        assert_eq!(result, "8".to_string());
    }
}
