use std::collections::HashSet;

fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_delta(d: char) -> (isize, isize) {
    match d {
        'U' => (-1, 0),
        'R' => (0, 1),
        'D' => (1, 0),
        'L' => (0, -1),
        _ => panic!("unknown direction"),
    }
}

fn get_neighbors(
    min_r: isize,
    max_r: isize,
    min_c: isize,
    max_c: isize,
    r: isize,
    c: isize,
) -> Vec<(isize, isize)> {
    let mut result: Vec<(isize, isize)> = Vec::new();
    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nr = r + dr;
        let nc = c + dc;
        if nr >= min_r && nr < max_r && nc >= min_c && nc < max_c {
            result.push((nr, nc));
        }
    }
    result
}

fn fill(input_parts: &Vec<(char, usize, String)>) -> isize {
    let mut r: isize = 0;
    let mut c: isize = 0;
    let mut min_r: isize = 0;
    let mut min_c: isize = 0;
    let mut max_r: isize = 0;
    let mut max_c: isize = 0;
    let mut grid: HashSet<(isize, isize)> = HashSet::new();
    let mut filled: HashSet<(isize, isize)> = HashSet::new();
    grid.insert((0, 0));

    for (d, l, _s) in input_parts {
        let (dr, dc) = get_delta(*d);
        for _ in 0..*l {
            r += dr;
            c += dc;
            grid.insert((r, c));
        }
        min_r = min_r.min(r);
        max_r = max_r.max(r);
        min_c = min_c.min(c);
        max_c = max_c.max(c);
    }
    min_r -= 1;
    min_c -= 1;
    max_r += 2;
    max_c += 2;

    let mut q: Vec<(isize, isize)> = Vec::new();
    for i in min_r..max_r {
        q.push((i, min_c));
        q.push((i, max_c - 1));
    }
    for i in min_c..max_c {
        q.push((min_r, i));
        q.push((max_r - 1, i));
    }
    while let Some((r, c)) = q.pop() {
        if filled.contains(&(r, c)) {
            continue;
        }
        filled.insert((r, c));
        for (nr, nc) in get_neighbors(min_r, max_r, min_c, max_c, r, c) {
            if !grid.contains(&(nr, nc)) {
                q.push((nr, nc));
            }
        }
    }
    (max_r - min_r) * (max_c - min_c) - filled.len() as isize
}

fn part1(input: &str) -> isize {
    let mut input_parts: Vec<(char, usize, String)> = Vec::new();
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(' ').collect();
        input_parts.push((
            parts[0].chars().next().unwrap(),
            parts[1].parse().ok().unwrap(),
            (parts[2].to_string()[1..parts[2].len() - 1]).to_string(),
        ));
    });
    fill(&input_parts)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let result = part1(test_case);
        assert_eq!(result, 62);
    }
}
