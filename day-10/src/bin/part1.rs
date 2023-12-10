use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();
    let s_index: usize = input.find('S').unwrap();
    let s_row = s_index / (cols + 1);
    let s_col = s_index % (cols + 1);

    //assume no boarder case
    if grid[s_row + 1][s_col] == 'L'
        || grid[s_row + 1][s_col] == 'J'
        || grid[s_row + 1][s_col] == '|'
    {
        if grid[s_row][s_col + 1] == 'J'
            || grid[s_row][s_col + 1] == '7'
            || grid[s_row + 1][s_col] == '-'
        {
            grid[s_row][s_col] = 'F';
        }
        if grid[s_row][s_col - 1] == 'F'
            || grid[s_row][s_col - 1] == 'L'
            || grid[s_row + 1][s_col] == '-'
        {
            grid[s_row][s_col] = '7';
        }
        if grid[s_row - 1][s_col] == 'F'
            || grid[s_row - 1][s_col] == '|'
            || grid[s_row - 1][s_col] == '7'
        {
            grid[s_row][s_col] = '|';
        }
    } else if grid[s_row - 1][s_col] == 'F'
        || grid[s_row - 1][s_col] == '|'
        || grid[s_row - 1][s_col] == '7'
    {
        if grid[s_row][s_col + 1] == 'J'
            || grid[s_row][s_col + 1] == '7'
            || grid[s_row + 1][s_col] == '-'
        {
            grid[s_row][s_col] = 'L';
        }
        if grid[s_row][s_col - 1] == 'F'
            || grid[s_row][s_col - 1] == 'L'
            || grid[s_row + 1][s_col] == '-'
        {
            grid[s_row][s_col] = 'J';
        }
    } else {
        grid[s_row][s_col] = '-';
    }

    println!("grid {:?}", grid);
    println!("s_index {}, s_row {}, s_col {}", s_index, s_row, s_col);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut deque: VecDeque<(usize, usize, usize)> = VecDeque::new();
    deque.push_back((0, s_row, s_col));
    let mut steps = 0;
    while !deque.is_empty() {
        let Some((step, row, col)) = deque.pop_front() else {
            panic!()
        };
        if visited.contains(&(row, col)) {
            continue;
        }
        steps = cmp::max(steps, step);
        visited.insert((row, col));
        if grid[row][col] == '|' {
            if row > 0 {
                deque.push_back((step + 1, row - 1, col));
            }
            if row < rows - 1 {
                deque.push_back((step + 1, row + 1, col));
            }
        }
        if grid[row][col] == '-' || grid[row][col] == 'S' {
            //6831
            if col > 0 {
                deque.push_back((step + 1, row, col - 1));
            }
            if col < cols - 1 {
                deque.push_back((step + 1, row, col + 1));
            }
        }
        if grid[row][col] == 'L' {
            if row > 0 {
                deque.push_back((step + 1, row - 1, col));
            }
            if col < cols - 1 {
                deque.push_back((step + 1, row, col + 1));
            }
        }
        if grid[row][col] == 'J' {
            if row > 0 {
                deque.push_back((step + 1, row - 1, col));
            }
            if col > 0 {
                deque.push_back((step + 1, row, col - 1));
            }
        }
        if grid[row][col] == '7' {
            if row < rows - 1 {
                deque.push_back((step + 1, row + 1, col));
            }
            if col > 0 {
                deque.push_back((step + 1, row, col - 1));
            }
        }
        if grid[row][col] == 'F' {
            if row < rows - 1 {
                deque.push_back((step + 1, row + 1, col));
            }
            if col < cols - 1 {
                deque.push_back((step + 1, row, col + 1));
            }
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "...F7.
..FJ|.
.SJ.L7
.|F--J
.LJ...";
        let result = part1(test_case);
        assert_eq!(result, 8);
    }
}
