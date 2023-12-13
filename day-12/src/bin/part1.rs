fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_combinations_start(s: &[char], t: &[usize], _i: usize, _j: usize) -> isize {
    let mut memo = vec![vec![-1; t.len() + 1]; s.len() + 1];

    fn get_combinations(
        memo: &mut Vec<Vec<isize>>,
        s: &[char],
        t: &[usize],
        i: usize,
        j: usize,
    ) -> isize {
        //println!("i{},j{}",i,j);
        if memo[i][j] != -1 {
            return memo[i][j];
        }
        if j == t.len() {
            if s.iter().skip(i).any(|&c| c == '#') {
                memo[i][j] = 0_isize;
                return 0;
            } else {
                memo[i][j] = 1_isize;
                return 1;
            }
        }
        if s.len() == i {
            memo[i][j] = 0_isize;
            return 0;
        }
        let mut result = 0;
        let remain: usize = t.iter().skip(j).sum::<usize>() + t.len() - j - 1;

        for k in i..(s.len() - remain + 1) {
            if s[k] == '.' {
                continue;
            }
            if !s.iter().skip(k).take(t[j]).any(|&c| c == '.') {
                if k + t[j] == s.len() {
                    result += get_combinations(memo, s, t, k + t[j], j + 1);
                } else if s[k + t[j]] != '#' {
                    result += get_combinations(memo, s, t, k + t[j] + 1, j + 1);
                }
            }
            if s[k] == '#' {
                break;
            }
        }
        memo[i][j] = result;
        result
    }
    get_combinations(&mut memo, s, t, 0, 0)
}

fn part1(input: &str) -> isize {
    let numbers: Vec<isize> = input
        .lines()
        .map(|line: &str| {
            let sp: Vec<&str> = line.split(' ').collect();
            let s: Vec<char> = sp[0].chars().collect();
            let t: Vec<usize> = sp[1].split(',').map(|c| c.parse().unwrap()).collect();
            get_combinations_start(&s, &t, 0, 0)
        })
        .collect();
    numbers.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = "???.### 1,1,3";
        let result = part1(test_case);
        assert_eq!(result, 1);
    }
    #[test]
    fn it_works_2() {
        let test_case = ".??..??...?##. 1,1,3";
        let result = part1(test_case);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works_3() {
        let test_case = "?#?#?#?#?#?#?#? 1,3,1,6";
        let result = part1(test_case);
        assert_eq!(result, 1);
    }
    #[test]
    fn it_works_4() {
        let test_case = "????.#...#... 4,1,1";
        let result = part1(test_case);
        assert_eq!(result, 1);
    }
    #[test]
    fn it_works_5() {
        let test_case = "????.######..#####. 1,6,5";
        let result = part1(test_case);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works_6() {
        let test_case = "?###???????? 3,2,1";
        let result = part1(test_case);
        assert_eq!(result, 10);
    }
    #[test]
    fn it_works_all() {
        let test_case = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = part1(test_case);
        assert_eq!(result, 21);
    }
}
