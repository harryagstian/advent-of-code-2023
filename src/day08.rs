use std::collections::{HashMap, HashSet};

use color_eyre::eyre::Result;

use crate::solver::Answer;

#[derive(Debug)]
struct Map {
    instruction: Vec<usize>,
    nodes: HashMap<String, [String; 2]>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut vec = input.lines();
        let mut instruction = vec![];
        let mut nodes = HashMap::new();

        for c in vec.next().unwrap().chars() {
            let direction = match c {
                'L' => 0,
                'R' => 1,
                _ => unreachable!(),
            };

            instruction.push(direction);
        }

        assert_eq!(vec.next().unwrap().len(), 0);

        for line in vec {
            let vec: Vec<String> = line.split('=').map(|f| f.trim().to_string()).collect();
            assert_eq!(vec.len(), 2);

            let current = vec.first().unwrap();
            let node: Vec<String> = vec
                .last()
                .unwrap()
                .split(',')
                .map(|f| f.replace(['(', ')'], "").trim().to_string())
                .collect();

            assert_eq!(node.len(), 2);

            nodes.insert(current.clone(), [node[0].clone(), node[1].clone()]);
        }

        Map { instruction, nodes }
    }

    fn travel_to_zzz(&self) -> i32 {
        let mut current = "AAA".to_string();
        let mut index = 0;
        let mut steps = 0;

        while current != "ZZZ" {
            current = self.travel(index, current);

            steps += 1;
            index += 1; // move to next iteration
            index %= self.instruction.len() as u64; // prevent index out of bound
        }

        steps
    }

    fn travel(&self, index: u64, current: String) -> String {
        let direction = &self.instruction[index as usize];
        self.nodes.get(&current).unwrap()[*direction].clone()
    }

    fn travel_to_end_z(&self) -> u64 {
        // Least Common Multiple (LCM) problem
        // First, We need to determine the minimum denominator for each starting point

        let current_vec: Vec<String> = self
            .nodes
            .keys()
            .filter(|f| f.ends_with('A'))
            .map(|f| f.to_string())
            .collect();

        let mut numbers = vec![];

        for v in current_vec.iter() {
            let mut current = v.clone();
            let mut ends_with_z: HashSet<u64> = HashSet::new();

            let mut index = 0;
            let mut distance_traveled = 0;

            loop {
                distance_traveled += 1;
                current = self.travel(index, current.clone());

                if current.ends_with('Z') {
                    if ends_with_z.contains(&distance_traveled) {
                        break;
                    }
                    ends_with_z.insert(distance_traveled);
                    distance_traveled = 0;
                }

                index += 1;
                index %= self.instruction.len() as u64; // prevent index out of bound
            }

            let mut ends_with_z_vec: Vec<u64> = ends_with_z.into_iter().collect();
            ends_with_z_vec.sort();

            numbers.push(*ends_with_z_vec.first().unwrap());
        }

        numbers.iter().fold(1, |acc, &x| num::integer::lcm(acc, x))
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut answer = Answer::default();

    let map = Map::new(input);

    answer.part1 = Some(map.travel_to_zzz().to_string());
    answer.part2 = Some(map.travel_to_end_z().to_string());
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use crate::day08::Map;

    #[traced_test]
    #[test]
    fn test_part1() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let map = Map::new(input);

        assert_eq!(map.travel_to_zzz(), 6);
    }

    #[traced_test]
    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let map = Map::new(input);

        assert_eq!(map.travel_to_end_z(), 6);
    }
}
