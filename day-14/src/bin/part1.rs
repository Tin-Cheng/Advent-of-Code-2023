fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn count_load(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = 0;
    for i in 0..cols {
        let mut cur = rows;
        for (j, row) in grid.iter().enumerate().take(rows) {
            match row[i] {
                'O' => {
                    result += cur;
                    cur -= 1;
                }
                '#' => {
                    cur = rows - j - 1;
                }
                '.' => continue,
                _ => panic!("unknown char"),
            }
        }
    }
    result
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    count_load(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let result = part1(test_case);
        assert_eq!(result, 136);
    }
}
