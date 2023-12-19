use crate::solver::Answer;

use color_eyre::eyre::Result;

trait HashAlgorithmTrait {
    fn calculate(&self, item: &str) -> u32 {
        let mut value = 0;
        for c in item.chars() {
            value += c as u32;
            value *= 17;
            value %= 256;
        }

        value
    }
}

#[derive(Debug)]
struct HashAlgorithm {
    items: Vec<String>,
}

impl HashAlgorithmTrait for HashAlgorithm {}

impl HashAlgorithm {
    fn new(input: &str) -> Self {
        let items = input.trim().split(',').map(|f| f.to_string()).collect();

        Self { items }
    }

    fn calculate_all(&self) -> Vec<u32> {
        self.items.iter().map(|f| self.calculate(f)).collect()
    }
}

#[derive(Debug, Clone)]
enum HashMapOperation {
    Reduce,
    Upsert(u32),
}

impl HashMapOperation {
    fn get_focal_length(&self) -> u32 {
        match self {
            HashMapOperation::Upsert(value) => *value,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct HashMapItem {
    label: String,
    operation: HashMapOperation,
}

impl HashMapItem {
    fn new(input: &str) -> Self {
        let (label, operation) = if input.contains('-') {
            (input.replace('-', "").to_string(), HashMapOperation::Reduce)
        } else {
            let vec = input.split('=').collect::<Vec<&str>>();
            assert_eq!(vec.len(), 2);

            (
                vec.first().unwrap().to_string(),
                HashMapOperation::Upsert(vec.last().unwrap().parse::<u32>().unwrap()),
            )
        };

        Self { label, operation }
    }
}

#[derive(Debug)]
struct HashMapAlgorithm {
    items: Vec<HashMapItem>,
    boxes: Vec<Vec<HashMapItem>>,
}

impl HashAlgorithmTrait for HashMapAlgorithm {}

impl HashMapAlgorithm {
    fn new(input: &str) -> Self {
        let items = input.trim().split(',').map(HashMapItem::new).collect();

        Self {
            items,
            boxes: vec![Vec::new(); 256],
        }
    }

    fn execute_sequence(&mut self) {
        for item in &self.items {
            let box_index = self.calculate(&item.label);
            let current_box = &self.boxes[box_index as usize];

            match item.operation {
                HashMapOperation::Reduce => {
                    self.boxes[box_index as usize] = current_box
                        .iter()
                        .filter(|f| f.label != item.label)
                        .cloned()
                        .collect();
                }
                HashMapOperation::Upsert(_) => {
                    if let Some(index) = current_box.iter().position(|f| f.label == item.label) {
                        self.boxes[box_index as usize][index] = item.clone();
                    } else {
                        self.boxes[box_index as usize].push(item.clone())
                    }
                }
            }
        }
    }

    fn get_focusing_power(&self) -> u32 {
        let mut result = 0;

        for (box_index, current_box) in self.boxes.iter().enumerate() {
            for (lens_index, current_lens) in current_box.iter().enumerate() {
                result += (box_index as u32 + 1)
                    * (lens_index as u32 + 1)
                    * current_lens.operation.get_focal_length();
            }
        }

        result
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut answer = Answer::default();

    let hash_algorithm = HashAlgorithm::new(input);
    let part1 = hash_algorithm.calculate_all().iter().sum::<u32>();

    let mut hashmap_algorithm = HashMapAlgorithm::new(input);
    hashmap_algorithm.execute_sequence();
    let part2 = hashmap_algorithm.get_focusing_power();

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());
    Ok(answer)
}

#[cfg(test)]
mod tests {

    use tracing_test::traced_test;

    use super::*;
    use color_eyre::eyre::Result;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[traced_test]
    #[test]
    fn test_hash_algorithm_calculate_all() {
        let hash_algorithm = HashAlgorithm::new(TEST_INPUT);

        let result = [30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231];

        assert_eq!(hash_algorithm.calculate_all(), result);
    }

    #[traced_test]
    #[test]
    fn test_part1() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("1320".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("145".to_string()));

        Ok(())
    }
}
