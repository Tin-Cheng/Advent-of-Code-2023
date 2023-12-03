fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}
fn part1(input: &str) -> u32 {
    let rows: usize = input.chars().filter(|&c| c == '\n').count() + 1;
    let cols: usize = input
        .lines()
        .next()
        .expect("there should be at least one line")
        .chars()
        .count();

    let data: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    fn is_near_symbol(
        data: &Vec<Vec<char>>,
        rows: usize,
        cols: usize,
        row: usize,
        col: usize,
        length: usize,
    ) -> bool {
        let min_col = if col > 0 { col - 1 } else { 0 };
        let max_col = if col + length + 1 <= cols {
            col + length + 1
        } else {
            cols
        };

        if !data[row][min_col].is_digit(10) && data[row][min_col] != '.' {
            return true;
        }
        if !data[row][max_col - 1].is_digit(10) && data[row][max_col - 1] != '.' {
            return true;
        }
        if row > 0 {
            for c in min_col..max_col {
                if !data[row - 1][c].is_digit(10) && data[row - 1][c] != '.' {
                    return true;
                }
            }
        }
        if row + 1 < rows {
            for c in min_col..max_col {
                if !data[row + 1][c].is_digit(10) && data[row + 1][c] != '.' {
                    return true;
                }
            }
        }
        false
    }

    let mut result: u32 = 0;
    let mut row = 0;
    while row < rows {
        let mut col = 0;
        while col < cols {
            if data[row][col].is_digit(10) {
                let mut length = 0;
                let mut num: String = String::from("");
                while col + length < cols && data[row][col + length].is_digit(10) {
                    num = num + &String::from(data[row][col + length]);
                    length += 1;
                }
                if is_near_symbol(&data, rows, cols, row, col, length) {
                    result = result + num.parse::<u32>().unwrap();
                }
                col += length;
                println!("num {}, result {}", num, result);
            }
            col += 1;
        }
        row += 1;
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
        let result = part1(test_case);
        assert_eq!(result, 4361);
    }
}
