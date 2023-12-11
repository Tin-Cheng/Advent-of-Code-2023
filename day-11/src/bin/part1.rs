fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();
    //println!("rows {}, cols {}",rows,cols);
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();
    empty_rows.push(0);
    empty_cols.push(0);
    for r in 0..rows {
        let mut is_empty: bool = true;
        for c in 0..cols {
            if grid[r][c] == '#' {
                is_empty = false;
                galaxies.push((r, c));
            }
        }
        if is_empty {
            empty_rows.push(empty_rows.last().unwrap() + 1);
        } else {
            empty_rows.push(*empty_rows.last().unwrap());
        }
    }
    for c in 0..cols {
        let mut is_empty: bool = true;
        for r in 0..rows {
            if grid[r][c] == '#' {
                is_empty = false;
            }
        }
        if is_empty {
            empty_cols.push(empty_cols.last().unwrap() + 1);
        } else {
            empty_cols.push(*empty_cols.last().unwrap());
        }
    }

    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (ix, iy) = galaxies[i];
            let (jx, jy) = galaxies[j];
            result += (jx).abs_diff(ix)
                + (jy).abs_diff(iy)
                + empty_rows[jx + 1].abs_diff(empty_rows[ix])
                + empty_cols[jy + 1].abs_diff(empty_cols[iy]);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = part1(test_case);
        assert_eq!(result, 374);
    }
}
