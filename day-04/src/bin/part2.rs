use std::collections::HashMap;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut hash_map: HashMap<u32, u32> = HashMap::new();

    let numbers: Vec<u32> = input
        .lines()
        .map(|line: &str| {
            let line_str = line.to_string();
            let split: Vec<&str> = line_str.split(':').collect();

            let card_num: u32 = split[0]
                .trim()
                .split(' ')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let card_split: Vec<&str> = split[1].split('|').collect();
            let winning_card: Vec<&str> = card_split[0].trim().split(' ').collect();
            let number_card: Vec<&str> = card_split[1].trim().split(' ').collect();
            *hash_map.entry(card_num).or_default() += 1;

            let mut count = 0;
            for number in &number_card {
                if number == &"" {
                    continue;
                }
                if winning_card.contains(number) {
                    count += 1;
                }
            }
            for new_card in card_num + 1..card_num + 1 + count {
                *hash_map.entry(new_card).or_default() += *hash_map.get(&card_num).unwrap();
            }
            *hash_map.get(&card_num).unwrap()
        })
        .collect();

    //.collect();
    let result: u32 = numbers.into_iter().sum();
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part2(test_case);
        assert_eq!(result, 30);
    }
}
