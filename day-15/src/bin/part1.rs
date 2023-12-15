fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn hash(s: &str) -> isize {
    let mut cur: isize = 0;
    for c in s.chars() {
        let ascii_value: isize = c as isize;
        cur += ascii_value;
        cur *= 17;
        cur %= 256;
    }
    cur
}

fn part1(input: &str) -> isize {
    let result: Vec<isize> = input.split(',').map(|s: &str| hash(s)).collect();
    result.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = part1(test_case);
        assert_eq!(result, 1320);
    }
}
