fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn get_delta(d: char) -> (isize, isize) {
    match d {
        '3' => (-1, 0),
        '0' => (0, 1),
        '1' => (1, 0),
        '2' => (0, -1),
        _ => panic!("unknown direction"),
    }
}

fn shoelace(input_parts: &Vec<(char, usize, String)>) -> isize {
    let mut r: isize = 0;
    let mut c: isize = 0;
    let mut grid: Vec<(isize, isize)> = Vec::new();

    let mut parameter: isize = 0;
    for (_d, _l, s) in input_parts {
        let l_str = &s[1..s.len() - 1];
        let l = isize::from_str_radix(l_str, 16).unwrap();
        let d: char = s.chars().last().unwrap();
        println!("l {}, d {}", l, d);

        let (dr, dc) = get_delta(d);
        r += dr * l;
        c += dc * l;
        grid.push((r, c));
        parameter += l;
    }
    let mut s1: isize = 0;
    let mut s2: isize = 0;

    for i in 0..grid.len() - 1 {
        s1 += grid[i].0 * grid[i + 1].1;
        s2 += grid[i].1 * grid[i + 1].0;
    }
    s1 += grid[grid.len() - 1].0 * grid[0].1;
    s2 += grid[grid.len() - 1].1 * grid[0].0;
    ((s1 - s2).abs() + parameter) / 2 + 1
}

fn part2(input: &str) -> isize {
    let mut input_parts: Vec<(char, usize, String)> = Vec::new();
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(' ').collect();
        input_parts.push((
            parts[0].chars().next().unwrap(),
            parts[1].parse().ok().unwrap(),
            (parts[2].to_string()[1..parts[2].len() - 1]).to_string(),
        ));
    });
    shoelace(&input_parts)
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
        let result = part2(test_case);
        assert_eq!(result, 952408144115);
    }
}
