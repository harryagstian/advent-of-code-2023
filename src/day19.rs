use std::collections::HashMap;

use crate::solver::Answer;
use color_eyre::eyre::Result;
use regex::Regex;

#[derive(Debug)]
struct System {
    workflows: HashMap<String, Rule>,
    items: Vec<Item>,
}

impl System {
    fn new(input: &str) -> Self {
        enum Mode {
            Workflow,
            Input,
        }
        let mut mode = Mode::Workflow;
        let mut workflows = HashMap::new();
        let mut items = vec![];

        for line in input.lines() {
            if line.is_empty() {
                mode = Mode::Input;
                continue;
            }

            match mode {
                Mode::Workflow => {
                    let vec = line.split('{').collect::<Vec<&str>>();

                    assert_eq!(vec.len(), 2);

                    let id = vec[0].to_owned();
                    let rule_str = vec[1].replace('}', "").to_owned();

                    let rule = Rule::new(&rule_str);
                    workflows.insert(id, rule);
                }
                Mode::Input => {
                    let item = Item::new(line);
                    items.push(item);
                }
            }
        }

        Self { workflows, items }
    }

    fn get_accepted_value(&self) -> i32 {
        let mut total = 0;
        for item in &self.items {
            let mut current_id = "in";

            loop {
                let rule = self.workflows.get(current_id).unwrap();
                current_id = rule.process(item);

                if current_id == "A" || current_id == "R" {
                    if current_id == "A" {
                        total += item.get_total();
                    }

                    break;
                }
            }
        }

        total
    }
}

#[derive(Debug)]
struct Condition {
    category: Category,
    check: Check,
}

#[derive(Debug)]
struct Rule {
    conditions: Vec<Condition>,
    default: String,
}

impl Rule {
    fn new(rule_str: &str) -> Self {
        let mut conditions = vec![];
        let mut default = String::new();
        let mut iterator = rule_str.split(',').peekable();

        while let Some(item) = iterator.next() {
            if iterator.peek().is_none() {
                default = item.to_owned()
            } else {
                let re = Regex::new(r"([xmas])([<>])(\d*):(.*)").unwrap();
                let captures = re.captures(item).unwrap();

                assert_eq!(captures.len(), 5);

                let category = Category::new(captures.get(1).unwrap().as_str());

                let check = Check::new(
                    captures.get(2).unwrap().as_str(),
                    captures.get(3).unwrap().as_str(),
                    captures.get(4).unwrap().as_str(),
                );

                conditions.push(Condition { category, check });
            }
        }

        Self {
            conditions,
            default,
        }
    }

    fn process(&self, item: &Item) -> &str {
        for condition in &self.conditions {
            let item_value = item.component.get(&condition.category).unwrap();

            if condition.check.compare(*item_value) {
                return &condition.check.destination;
            }
        }

        &self.default
    }
}

#[derive(Debug)]
struct Check {
    op: String,
    value: i32,
    destination: String,
}

impl Check {
    fn new(op: &str, value: &str, destination: &str) -> Self {
        Self {
            op: op.to_owned(),
            value: value.parse().unwrap(),
            destination: destination.to_owned(),
        }
    }

    fn compare(&self, item_value: i32) -> bool {
        match self.op.as_str() {
            "<" => item_value < self.value,
            ">" => item_value > self.value,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn new(input: &str) -> Self {
        match input {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Item {
    component: HashMap<Category, i32>,
}

impl Item {
    fn new(input: &str) -> Self {
        let mut component = HashMap::new();

        for item in input.replace(['{', '}'], "").split(',') {
            let vec = item.split('=').collect::<Vec<&str>>();
            assert_eq!(vec.len(), 2);

            let category = Category::new(vec[0]);
            let value = vec[1].parse().unwrap();

            component.insert(category, value);
        }

        Self { component }
    }

    fn get_total(&self) -> i32 {
        self.component.values().sum()
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let part2 = 0;
    let mut answer = Answer::default();

    let system = System::new(input);
    let part1 = system.get_accepted_value();

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());
    Ok(answer)
}

#[cfg(test)]
mod tests {

    use tracing_test::traced_test;

    use super::*;
    use color_eyre::eyre::Result;

    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
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

    #[traced_test]
    #[test]
    fn test_part1() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("19114".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("".to_string()));

        Ok(())
    }
}
