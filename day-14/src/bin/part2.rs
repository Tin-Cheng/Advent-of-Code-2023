use std::collections::HashMap;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input, 1000000000);
    dbg!(output);
}

fn count_load(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = 0;
    for i in 0..cols {
        for (j, row) in grid.iter().enumerate().take(rows) {
            match row[i] {
                'O' => {
                    result += rows - j;
                }
                '#' => {
                    continue;
                }
                '.' => continue,
                _ => panic!("unknown char"),
            }
        }
    }
    result
}
fn move_rock(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut new_grid = vec![vec!['.'; rows]; cols];
    for i in 0..cols {
        let mut cur = 0;
        for (j, row) in grid.iter().enumerate().take(rows) {
            match row[i] {
                'O' => {
                    new_grid[cur][i] = 'O';
                    cur += 1;
                }
                '#' => {
                    new_grid[j][i] = '#';
                    cur = j + 1;
                }
                '.' => continue,
                _ => panic!("unknown char"),
            }
        }
    }
    new_grid
}
fn rotate(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut new_grid = vec![vec!['.'; rows]; cols];
    for (j, new_row) in new_grid.iter_mut().enumerate().take(cols) {
        for i in (0..rows).rev() {
            new_row[cols - i - 1] = grid[i][j];
        }
    }
    new_grid
}
fn simulate(grid: &[Vec<char>], cycles: usize) -> Vec<Vec<char>> {
    let mut cur_grid = grid.to_owned();
    let mut str_to_cycle: HashMap<String, usize> = HashMap::new();
    let mut cycle_to_str: HashMap<usize, String> = HashMap::new();

    for cycle in 0..cycles {
        for _ in 0..4 {
            cur_grid = rotate(&move_rock(&cur_grid));
        }
        let cur_grid_str: String = cur_grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        if str_to_cycle.contains_key(&cur_grid_str) {
            let previous_seen = str_to_cycle.get(&cur_grid_str).unwrap();
            let loop_length = cycle - previous_seen;
            let remaining = (cycles - cycle - 1) % loop_length;

            for _ in 0..remaining {
                for _ in 0..4 {
                    cur_grid = rotate(&move_rock(&cur_grid));
                }
            }
            return cur_grid;
        }
        str_to_cycle.insert(cur_grid_str.clone(), cycle);
        cycle_to_str.insert(cycle, cur_grid_str);
    }
    cur_grid
}
fn part2(input: &str, cycles: usize) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let new_grid: Vec<Vec<char>> = simulate(&grid, cycles);
    count_load(&new_grid)
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
        let result = part2(test_case, 1000000000);
        assert_eq!(result, 64);
    }
}
