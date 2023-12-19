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
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
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

fn check_combinations(condition: (char, char, isize), values: &[isize], b: bool) -> Vec<isize> {
    let mut new_values = values.to_owned();
    if condition.1 == '>' && b {
        match condition.0 {
            'x' => new_values[0] = condition.2 + 1,
            'm' => new_values[2] = condition.2 + 1,
            'a' => new_values[4] = condition.2 + 1,
            's' => new_values[6] = condition.2 + 1,
            _ => panic!("unknown value"),
        };
    } else if condition.1 == '>' && !b {
        match condition.0 {
            'x' => new_values[1] = condition.2,
            'm' => new_values[3] = condition.2,
            'a' => new_values[5] = condition.2,
            's' => new_values[7] = condition.2,
            _ => panic!("unknown value"),
        };
    } else if condition.1 == '<' && b {
        match condition.0 {
            'x' => new_values[1] = condition.2 - 1,
            'm' => new_values[3] = condition.2 - 1,
            'a' => new_values[5] = condition.2 - 1,
            's' => new_values[7] = condition.2 - 1,
            _ => panic!("unknown value"),
        };
    } else if condition.1 == '<' && !b {
        match condition.0 {
            'x' => new_values[0] = condition.2,
            'm' => new_values[2] = condition.2,
            'a' => new_values[4] = condition.2,
            's' => new_values[6] = condition.2,
            _ => panic!("unknown value"),
        };
    };
    new_values
}

fn get_part2(
    workflows: &HashMap<&str, Workflow>,
    values: &[isize],
    next: &crate::WorkflowTypes,
) -> isize {
    match next {
        WorkflowTypes::String(s) => {
            if let Some(workflow) = workflows.get(&s as &str) {
                get_part2(
                    workflows,
                    &check_combinations(workflow.condition, values, true),
                    &workflow.t,
                ) + get_part2(
                    workflows,
                    &check_combinations(workflow.condition, values, false),
                    &workflow.f,
                )
            } else {
                panic!("unknown workflow {}", s);
            }
        }
        WorkflowTypes::Bool(b) => {
            if *b
                && values[1] > values[0]
                && values[3] > values[2]
                && values[5] > values[4]
                && values[7] > values[6]
            {
                return (values[1] - values[0] + 1)
                    * (values[3] - values[2] + 1)
                    * (values[5] - values[4] + 1)
                    * (values[7] - values[6] + 1);
            }
            0
        }
        WorkflowTypes::Struct(workflow) => {
            get_part2(
                workflows,
                &check_combinations(workflow.condition, values, true),
                &workflow.t,
            ) + get_part2(
                workflows,
                &check_combinations(workflow.condition, values, false),
                &workflow.f,
            )
        }
    }
}

fn part2(input: &str) -> isize {
    let data: Vec<&str> = input.split("\n\n").collect();
    let workflows = get_workflows(data[0]);
    let p2v: Vec<isize> = vec![1, 4000, 1, 4000, 1, 4000, 1, 4000];
    get_part2(&workflows, &p2v, &WorkflowTypes::String("in".to_string()))
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
        let result = part2(test_case);
        assert_eq!(result, 167409079868000);
    }
}
