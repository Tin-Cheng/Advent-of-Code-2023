fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}
fn is_mirror(grid: &Vec<Vec<char>>, i: usize, is_vertical: bool) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    if !is_vertical {
        let distance: usize = i.min(cols - i);
        let mut all_same = true;
        for j in 0..distance {
            for row in grid.iter().take(rows) {
                if row[i - j - 1] != row[i + j] {
                    all_same = false;
                    break;
                }
            }
        }
        all_same
    } else {
        let distance: usize = i.min(rows - i);
        let mut all_same = true;
        for j in 0..distance {
            for c in 0..cols {
                if grid[i - j - 1][c] != grid[i + j][c] {
                    all_same = false;
                    break;
                }
            }
        }
        all_same
    }
}

fn search_mirror(m: &str) -> usize {
    let grid: Vec<Vec<char>> = m.split('\n').map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    for i in 1..cols {
        if is_mirror(&grid, i, false) {
            return i;
        }
    }
    for i in 1..rows {
        if is_mirror(&grid, i, true) {
            return i * 100;
        }
    }
    0
}

fn part1(input: &str) -> usize {
    let result: Vec<usize> = input.split("\n\n").map(search_mirror).collect();
    result.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let result = part1(test_case);
        assert_eq!(result, 405);
    }
}
