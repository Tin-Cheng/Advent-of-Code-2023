//https://gist.github.com/samueltardieu/ff6cd6cd284af1179ab20f8732ee5e93
use ndarray::prelude::*;
use ndarray_linalg::Solve;

type Pos = (i64, i64, i64);
type Hailstone = (Pos, Pos);

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i64 {
    let input_clean = input.replace(' ', "");
    let hailstones: Vec<Hailstone> = input_clean
        .lines()
        .map(|line: &str| {
            let ns = line.split([',', '@'])
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|s| s.parse::<i64>().unwrap())
                .take(6)
                .collect::<Vec<_>>();
            ((ns[0], ns[1], ns[2]), (ns[3], ns[4], ns[5]))
        })
        .collect();
    let sx = speed_x(
        hailstones
            .iter()
            .map(|((x, _, _), (dx, _, _))| (*x, *dx))
            .collect(),
    ) as f64;
    let sy = speed_x(
        hailstones
            .iter()
            .map(|((_, y, _), (_, dy, _))| (*y, *dy))
            .collect(),
    ) as f64;
    let sz = speed_x(
        hailstones
            .iter()
            .map(|((_, _, z), (_, _, dz))| (*z, *dz))
            .collect(),
    ) as f64;
    let a: Array2<f64> = array![
        [1.0, 0.0, 0.0, sx - hailstones[0].1 .0 as f64, 0.0],
        [0.0, 1.0, 0.0, sy - hailstones[0].1 .1 as f64, 0.0],
        [0.0, 0.0, 1.0, sz - hailstones[0].1 .2 as f64, 0.0],
        [1.0, 0.0, 0.0, 0.0, sx - hailstones[1].1 .0 as f64],
        [0.0, 1.0, 0.0, 0.0, sy - hailstones[1].1 .1 as f64]
    ];

    let b: Array1<f64> = array![
        hailstones[0].0 .0 as f64,
        hailstones[0].0 .1 as f64,
        hailstones[0].0 .2 as f64,
        hailstones[1].0 .0 as f64,
        hailstones[1].0 .1 as f64
    ];
    let x = a.solve_into(b).unwrap();
    (x[0] + x[1] + x[2]).round() as i64
}

fn speed_x(xs: Vec<(i64, i64)>) -> i64 {
    let min_bound = xs.iter().map(|(_, s)| *s).min().unwrap();
    let max_bound = xs.iter().map(|(_, s)| *s).max().unwrap();
    let mut possible = Vec::new();
    possible.resize((max_bound - min_bound + 1) as usize, true);
    for (i, (x, v)) in xs.iter().enumerate() {
        for (xx, vv) in xs.iter().skip(i + 1) {
            if v == vv {
                let dist = (x - xx).abs();
                for i in 0..possible.len() {
                    let ss = i as i64 + min_bound - *vv as i64;
                    if ss != 0 && possible[i] && dist % ss != 0 {
                        possible[i] = false;
                    }
                }
            }
        }
    }
    possible.into_iter().position(|s| s).unwrap() as i64 + min_bound
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#;
        let result = part2(test_case);
        assert_eq!(result, 47);
    }
}
