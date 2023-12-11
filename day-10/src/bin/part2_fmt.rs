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

    let get_neighbors = |pipe: char| -> Vec<(isize, isize)> {
        let vec = match pipe {
            '|' => vec![(-1, 0), (1, 0)],
            '-' => vec![(0, -1), (0, 1)],
            'L' => vec![(-1, 0), (0, 1)],
            'J' => vec![(-1, 0), (0, -1)],
            '7' => vec![(0, -1), (1, 0)],
            'F' => vec![(1, 0), (0, 1)],
            '.' => vec![],
            _ => panic!("wrong pipe {}", pipe),
        };
        vec
    };

    let get_valid_neighbors = |r: usize, c: usize| -> Vec<(usize, usize)> {
        let neighbors = get_neighbors(grid[r][c]);
        let mut vec: Vec<(usize, usize)> = Vec::new();
        for (dr, dc) in neighbors {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                vec.push((nr as usize, nc as usize));
            }
        }
        vec
    };

    //assume no boarder case
    let up_neighbors = get_valid_neighbors(s_row - 1,s_col);
    let down_neighbors = get_valid_neighbors(s_row + 1,s_col);
    let left_neighbors = get_valid_neighbors(s_row,s_col - 1);
    let right_neighbors = get_valid_neighbors(s_row,s_col + 1);

    if up_neighbors.contains(&(s_row,s_col)) && down_neighbors.contains(&(s_row,s_col)){
        grid[s_row][s_col] = '|';
    }
    else if up_neighbors.contains(&(s_row,s_col)) && right_neighbors.contains(&(s_row,s_col)){
        grid[s_row][s_col] = 'L';
    }
    else if up_neighbors.contains(&(s_row,s_col)) && left_neighbors.contains(&(s_row,s_col)){
        grid[s_row][s_col] = 'J';
    }
    else if down_neighbors.contains(&(s_row,s_col)) && right_neighbors.contains(&(s_row,s_col)){
        grid[s_row][s_col] = 'F';
    }
    else if down_neighbors.contains(&(s_row,s_col)) && left_neighbors.contains(&(s_row,s_col)){
        grid[s_row][s_col] = '7';
    }
    else if left_neighbors.contains(&(s_row,s_col)) && right_neighbors.contains(&(s_row,s_col)){
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
        let neighbors = get_valid_neighbors(row, col);
        for (row, col) in neighbors {
            deque.push_back((step + 1, row, col));
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
