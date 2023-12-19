use std::collections::HashMap;
enum WorkflowTypes {
    Struct(Box<Workflow>),
    Bool(bool),
    String(String),
}
struct Workflow {
    condition: (char, char, isize),
    t: crate::WorkflowTypes,
    f: crate::WorkflowTypes,
}

fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn create_workflow_type(input: &str) -> crate::WorkflowTypes {
    if input == "A" {
        return WorkflowTypes::Bool(true);
    }
    if input == "R" {
        return WorkflowTypes::Bool(false);
    }
    if input.contains(':') {
        return WorkflowTypes::Struct(Box::new(create_workflow(input)));
    }
    WorkflowTypes::String(input.to_string())
}

fn create_workflow(s: &str) -> Workflow {
    let s_c: Vec<char> = s.chars().collect();
    let Some(colon) = s.find(':') else { panic!() };
    let Some(comma) = s.find(',') else { panic!() };
    let l = s.len();
    let num: isize = s[2..colon].parse::<isize>().unwrap();
    let _t: crate::WorkflowTypes = create_workflow_type(&s[colon + 1..comma]);
    let _f: crate::WorkflowTypes = create_workflow_type(&s[comma + 1..l]);
    let con = (s_c[0], s_c[1], num);
    Workflow {
        condition: con,
        t: _t,
        f: _f,
    }
}
fn get_workflows(input: &str) -> HashMap<&str, Workflow> {
    let mut workflows = HashMap::new();
    input.lines().for_each(|line| {
        let Some(open_brace) = line.find('{') else {
            panic!()
        };
        let Some(close_brace) = line.find('}') else {
            panic!()
        };
        let key: &str = &line[..open_brace];
        let workflow = create_workflow(&line[open_brace + 1..close_brace]);
        workflows.insert(key, workflow);
    });
    workflows
}

fn compare_condition(condition: (char, char, isize), values: &[isize]) -> bool {
    let value = match condition.0 {
        'x' => values[0],
        'm' => values[1],
        'a' => values[2],
        's' => values[3],
        _ => panic!("unknown value"),
    };
    if condition.1 == '>' {
        value > condition.2
    } else {
        value < condition.2
    }
}

fn check_workflow(
    workflows: &HashMap<&str, Workflow>,
    values: &Vec<isize>,
    next: &crate::WorkflowTypes,
) -> bool {
    match next {
        WorkflowTypes::String(s) => {
            if let Some(workflow) = workflows.get(&s as &str) {
                if compare_condition(workflow.condition, values) {
                    check_workflow(workflows, values, &workflow.t)
                } else {
                    check_workflow(workflows, values, &workflow.f)
                }
            } else {
                panic!("unknown workflow {}", s);
            }
        }
        WorkflowTypes::Bool(b) => *b,
        WorkflowTypes::Struct(workflow) => {
            if compare_condition(workflow.condition, values) {
                check_workflow(workflows, values, &workflow.t)
            } else {
                check_workflow(workflows, values, &workflow.f)
            }
        }
    }
}

fn part1(input: &str) -> isize {
    let data: Vec<&str> = input.split("\n\n").collect();
    let workflows = get_workflows(data[0]);

    let ratings: Vec<Vec<isize>> = data[1]
        .replace(['{', '}', '=', 'x', 'm', 'a', 's'], "")
        .lines()
        .map(|line: &str| {
            line.split(',')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .collect();
    let mut result: isize = 0;
    ratings.iter().for_each(|values| {
        if check_workflow(&workflows, values, &WorkflowTypes::String("in".to_string())) {
            result += values[0] + values[1] + values[2] + values[3];
        }
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let result = part1(test_case);
        assert_eq!(result, 19114);
    }
}
