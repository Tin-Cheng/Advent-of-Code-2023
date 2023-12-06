fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i64 {
    let data: Vec<&str> = input.lines().collect();

    let time: Vec<f64> = data[0]
        .split(':')
        .collect::<Vec<&str>>()
        .last()
        .expect("should have data")
        .trim()
        .split(' ')
        .map(|s| s.trim().parse::<f64>())
        .filter_map(Result::ok)
        .collect();

    let distance: Vec<f64> = data[1]
        .split(':')
        .collect::<Vec<&str>>()
        .last()
        .expect("should have data")
        .trim()
        .split(' ')
        .map(|s| s.trim().parse::<f64>())
        .filter_map(Result::ok)
        .collect();

    println!("time {:?}, distance {:?}", time, distance);

    let mut result: i64 = 1;

    for race in time.iter().zip(distance.iter()) {
        let (t, d) = race;
        //let hold time = s
        //find s * (t - s) > d
        // -s^2 + st - d > 0
        //apply ax^2 + bx + c = 0
        // a = -1, b = t, c = -d
        // x = (-t +- sqrt(t^2 - 4d)) / -2
        let ans1_float = (-t + (t * t - 4.0 * d).sqrt()) / -2.0;
        let ans2_float = (-t - (t * t - 4.0 * d).sqrt()) / -2.0;
        let mut ans1: i64 = ans1_float.ceil() as i64;
        let mut ans2: i64 = ans2_float.floor() as i64;
        if ans1_float == ans1 as f64 {
            ans1 += 1;
        }
        if ans2_float == ans2 as f64 {
            ans2 -= 1;
        }
        println!("race:{:?},1:{}, 2:{}", race, ans1, ans2);
        result *= ans2 - ans1 + 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "Time:      7  15   30
Distance:  9  40  200";
        let result = part1(test_case);
        assert_eq!(result, 288);
    }
}
