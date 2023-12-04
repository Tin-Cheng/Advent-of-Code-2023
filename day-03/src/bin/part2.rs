use std::collections::HashMap;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}
fn part2(input: &str) -> u32 {
    let rows: usize = input.chars().filter(|&c| c == '\n').count() + 1;
    let cols: usize = input
        .lines()
        .next()
        .expect("there should be at least one line")
        .chars()
        .count();

    let data: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let mut hash_map: HashMap<String, Vec<u32>> = HashMap::new();

    fn is_near_symbol(
        data: &[Vec<char>],
        rows: usize,
        cols: usize,
        row: usize,
        col: usize,
        length: usize,
    ) -> bool {
        let min_col = if col > 0 { col - 1 } else { 0 };
        let max_col = if col + length < cols {
            col + length + 1
        } else {
            cols
        };

        if !data[row][min_col].is_ascii_digit() && data[row][min_col] != '.' {
            return true;
        }
        if !data[row][max_col - 1].is_ascii_digit() && data[row][max_col - 1] != '.' {
            return true;
        }
        if row > 0 {
            for c in min_col..max_col {
                if !data[row - 1][c].is_ascii_digit() && data[row - 1][c] != '.' {
                    return true;
                }
            }
        }
        if row + 1 < rows {
            for c in min_col..max_col {
                if !data[row + 1][c].is_ascii_digit() && data[row + 1][c] != '.' {
                    return true;
                }
            }
        }
        false
    }

    fn check_nearby_gears(
        data: &[Vec<char>],
        rows: usize,
        cols: usize,
        row: usize,
        col: usize,
        length: usize,
    ) -> Vec<String> {
        let mut result = Vec::new();
        let min_col = if col > 0 { col - 1 } else { 0 };
        let max_col = if col + length < cols {
            col + length + 1
        } else {
            cols
        };

        if data[row][min_col] == '*' {
            result.push(row.to_string() + "_" + &min_col.to_string());
        }
        if data[row][max_col - 1] == '*' {
            result.push(row.to_string() + "_" + &(max_col - 1).to_string());
        }
        if row > 0 {
            for c in min_col..max_col {
                if data[row - 1][c] == '*' {
                    result.push((row - 1).to_string() + "_" + &c.to_string());
                }
            }
        }
        if row + 1 < rows {
            for c in min_col..max_col {
                if data[row + 1][c] == '*' {
                    result.push((row + 1).to_string() + "_" + &c.to_string());
                }
            }
        }
        result
    }

    let mut result: u32 = 0;
    let mut row = 0;
    while row < rows {
        let mut col = 0;
        while col < cols {
            if data[row][col].is_ascii_digit() {
                let mut length = 0;
                let mut num: String = String::from("");
                while col + length < cols && data[row][col + length].is_ascii_digit() {
                    num = num + &String::from(data[row][col + length]);
                    length += 1;
                }
                /*if is_near_symbol(&data, rows, cols, row, col, length) {
                    result = result + num.parse::<u32>().unwrap();
                }*/
                let nearby_gears = check_nearby_gears(&data, rows, cols, row, col, length);
                for gear in &nearby_gears {
                    hash_map
                        .entry(gear.to_string())
                        .or_default()
                        .push(num.parse::<u32>().unwrap());
                }
                col += length;
            }
            col += 1;
        }
        row += 1;
    }

    for value in hash_map.values() {
        if value.len() == 2 {
            result += value[0] * value[1];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part2(test_case);
        assert_eq!(result, 467835);
    }
}
