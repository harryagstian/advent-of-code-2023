use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Range,
    str::FromStr,
    u64,
};

use color_eyre::eyre::Result;
use strum::{Display, EnumString};
use tracing::info;

use crate::solver::Answer;

enum Phase {
    Initial,
    Map,
}

#[derive(EnumString, Debug, PartialEq, Eq, Clone)]
enum Category {
    #[strum(ascii_case_insensitive)]
    Seed,
    #[strum(ascii_case_insensitive)]
    Soil,
    #[strum(ascii_case_insensitive)]
    Fertilizer,
    #[strum(ascii_case_insensitive)]
    Water,
    #[strum(ascii_case_insensitive)]
    Light,
    #[strum(ascii_case_insensitive)]
    Temperature,
    #[strum(ascii_case_insensitive)]
    Humidity,
    #[strum(ascii_case_insensitive)]
    Location,
}

#[derive(Debug)]
struct Almanac {
    seeds_one: Vec<Range<u64>>,
    seeds_range: Vec<Range<u64>>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    source_category: Category,
    destination_category: Category,
    source_category_range: HashMap<usize, Range<u64>>,
    destination_category_range: HashMap<usize, Range<u64>>,
}

impl Map {
    fn new(mut input: VecDeque<String>) -> Self {
        assert!(input.len() > 1);

        // first line is always contains source / destination category
        let first_line = input.pop_front().unwrap();

        let mut vec = first_line.split("-to-").collect::<Vec<&str>>();

        assert_eq!(vec.len(), 2);

        // get category from the string
        let last = vec.pop().unwrap();
        let first = vec.pop().unwrap();
        let source_category = Category::from_str(first).unwrap();
        let destination_category = Category::from_str(last).unwrap();

        let mut source_category_range = HashMap::new();
        let mut destination_category_range = HashMap::new();

        // parse all number ranges
        for (index, line) in input.iter().enumerate() {
            let mut line = line
                .split_whitespace()
                .map(|f| f.parse().unwrap())
                .collect::<Vec<u64>>();

            assert_eq!(line.len(), 3);

            let interval = line.pop().unwrap();
            let src = line.pop().unwrap();
            let dst = line.pop().unwrap();

            source_category_range.insert(index, src..src + interval);

            destination_category_range.insert(index, dst..dst + interval);
        }

        Self {
            source_category,
            destination_category,
            source_category_range,
            destination_category_range,
        }
    }

    fn get_next_value(&self, value: u64) -> u64 {
        for (key, source_range) in self.source_category_range.iter() {
            if source_range.contains(&value) {
                let destination_range = self.destination_category_range.get(&key).unwrap();
                let diff = value - source_range.start;

                return destination_range.start + diff;
            }
        }

        value
    }
}

impl Almanac {
    fn new(input: &str) -> Self {
        let mut seeds_one = vec![];
        let mut seeds_range = vec![];
        let mut maps = vec![];

        let mut line_iter = input.lines().into_iter();

        while let Some(line) = line_iter.next() {
            if line.len() == 0 {
                continue;
            }

            // handle first line, it should always has initial seeds
            if seeds_one.len() == 0 {
                let v = line.replace("seeds:", "").trim().to_string();
                let mut start = 0;
                let mut end;

                for (index, x) in v.split_whitespace().map(|f| f.parse().unwrap()).enumerate() {
                    seeds_one.push(x..x + 1);

                    if index % 2 == 0 {
                        start = x;
                    } else {
                        end = x;
                        seeds_range.push(start..start + end);
                    }
                }
            }

            assert!(seeds_one.len() > 0);

            if line.contains("map:") {
                let mut map_stacks = VecDeque::from([line.replace("map:", "").trim().to_string()]);

                while let Some(l) = line_iter.next() {
                    if l.len() == 0 {
                        break;
                    }

                    map_stacks.push_back(l.to_string());
                }

                let map = Map::new(map_stacks);
                maps.push(map);
            }
        }

        Self {
            seeds_one,
            seeds_range,
            maps,
        }
    }

    fn lookup(&self, value: u64, source_category: Category) -> (u64, Category) {
        let map = self
            .maps
            .iter()
            .find(|f| f.source_category == source_category)
            .unwrap();

        let next_value = map.get_next_value(value);

        (next_value, map.destination_category.clone())
    }

    fn solve(&self, seeds: &Vec<Range<u64>>) -> u64 {
        let mut stacks = vec![];

        for range in seeds.iter() {
            for v in range.clone() {
                // dbg!(&v);
                let mut source_category = Category::Seed;
                let mut value = v;

                while source_category != Category::Location {
                    (value, source_category) = self.lookup(value, source_category);
                }
                stacks.push(value);
            }
        }

        stacks.sort();
        // dbg!(&stacks);
        *stacks.first().unwrap()
    }
}

pub fn solve_day05(input: &str) -> Result<Answer> {
    let almanac = Almanac::new(input);

    let part1 = almanac.solve(&almanac.seeds_one);
    let part2 = almanac.solve(&almanac.seeds_range);

    dbg!(&almanac);

    let answer = Answer {
        part1: Some(part1.to_string()),
        part2: Some(part2.to_string()),
    };

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use crate::day05::Almanac;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_part1() {
        let almanac = Almanac::new(TEST_INPUT);
        let solution = almanac.solve(&almanac.seeds_one);
        assert_eq!(solution, 35);
    }

    #[test]
    fn test_part2() {
        let almanac = Almanac::new(TEST_INPUT);
        let solution = almanac.solve(&almanac.seeds_range);
        assert_eq!(solution, 46);
    }
}
