use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
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
        if grid[row][col] == 'S' {
            if row > 0 {
                deque.push_back((step + 1, row - 1, col));
            }
            if row < rows - 1 {
                deque.push_back((step + 1, row + 1, col));
            }
            if col > 0 {
                deque.push_back((step + 1, row, col - 1));
            }
            if col < cols - 1 {
                deque.push_back((step + 1, row, col + 1));
            }
        }
        if grid[row][col] == '|' {
            if row > 0 {
                deque.push_back((step + 1, row - 1, col));
            }
            if row < rows - 1 {
                deque.push_back((step + 1, row + 1, col));
            }
        }
        if grid[row][col] == '-' {
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
    let mut count: usize = 0;
    for (r, row) in grid.iter().enumerate().take(rows) {
        let mut is_inside: bool = false;
        let mut last_char: char = ' ';
        for (c, v) in row.iter().enumerate().take(cols) {
            if visited.contains(&(r, c)) {
                if *v == '|' {
                    is_inside = !is_inside;
                }
                if *v == 'F' || *v == 'L' {
                    last_char = *v;
                }
                if (last_char == 'F' && *v == 'J') || (last_char == 'L' && *v == '7') {
                    is_inside = !is_inside;
                }
            } else if is_inside {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let result = part2(test_case);
        assert_eq!(result, 8);
    }
}
