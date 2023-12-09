use std::{collections::VecDeque, i64, str::FromStr};

use color_eyre::eyre::Result;
use num_traits::{PrimInt, Zero};
use strum::EnumString;

use crate::solver::Answer;

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
    seeds_one: Vec<Range<i64>>,
    seeds_range: Vec<Range<i64>>,
    maps: Vec<Map>,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Range<T> {
    start: T,
    end: T,
    diff: T,
}

impl<T> Range<T> {
    fn new(start: T, end: T, diff: T) -> Self {
        Self { start, end, diff }
    }
}

trait FillGaps {
    fn fill_gaps(&mut self);
}

impl<T: PrimInt + std::fmt::Debug> FillGaps for Vec<Range<T>> {
    fn fill_gaps(&mut self) {
        let iter = self.iter().peekable();
        let mut min_value = Zero::zero();

        let mut new_vec = vec![];

        for current in iter {
            if current.start > min_value {
                new_vec.push(Range {
                    start: min_value,
                    end: current.start,
                    diff: Zero::zero(),
                })
            }
            new_vec.push(current.clone());

            min_value = current.end;
        }

        new_vec.push(Range {
            start: min_value,
            end: T::max_value(),
            diff: Zero::zero(),
        });

        *self = new_vec;
    }
}

#[derive(Debug)]
struct Map {
    source_category: Category,
    destination_category: Category,
    formulas: Vec<Range<i64>>,
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
        let mut formulas = vec![];

        // parse all number ranges
        for line in input.iter() {
            let mut line = line
                .split_whitespace()
                .map(|f| f.parse().unwrap())
                .collect::<Vec<i64>>();

            assert_eq!(line.len(), 3);

            let interval = line.pop().unwrap();
            let src = line.pop().unwrap();
            let dst = line.pop().unwrap();

            let formula = Range::new(src, src + interval, dst - src);
            formulas.push(formula);
        }

        formulas.sort();
        formulas.fill_gaps();

        Self {
            source_category,
            destination_category,
            formulas,
        }
    }
}

impl Almanac {
    fn new(input: &str) -> Self {
        let mut seeds_one = vec![];
        let mut seeds_range = vec![];
        let mut maps = vec![];

        let mut line_iter = input.lines();

        while let Some(line) = line_iter.next() {
            if line.is_empty() {
                continue;
            }

            // handle first line, it should always has initial seeds
            if seeds_one.is_empty() {
                let v = line.replace("seeds:", "").trim().to_string();
                let mut start = 0;
                let mut end;

                for (index, x) in v.split_whitespace().map(|f| f.parse().unwrap()).enumerate() {
                    seeds_one.push(Range::new(x, x + 1, 0));
                    if index % 2 == 0 {
                        start = x;
                    } else {
                        end = x;
                        seeds_range.push(Range::new(start, start + end, 0));
                    }
                }
            }

            assert!(!seeds_one.is_empty());

            if line.contains("map:") {
                let mut map_stacks = VecDeque::from([line.replace("map:", "").trim().to_string()]);

                for l in line_iter.by_ref() {
                    if l.is_empty() {
                        break;
                    }

                    map_stacks.push_back(l.to_string());
                }

                let map = Map::new(map_stacks);
                maps.push(map);
            }
        }

        seeds_one.sort();
        seeds_range.sort();

        Self {
            seeds_one,
            seeds_range,
            maps,
        }
    }

    fn get_next_range(
        &self,
        source_range: &Vec<Range<i64>>,
        source_category: Category,
    ) -> (Vec<Range<i64>>, Category) {
        let map = self
            .maps
            .iter()
            .find(|f| f.source_category == source_category)
            .unwrap();

        let mut result = vec![];

        for src in source_range {
            let mut new_range;
            for dst in map.formulas.iter() {
                // dbg!(&src, &dst);
                let diff = dst.diff;
                if src.start >= dst.start && src.end <= dst.end {
                    // src is subset of dst
                    new_range = Range::new(src.start + diff, src.end + diff, 0);
                } else if src.start < dst.start && src.end > dst.end {
                    // src is superset of dst
                    new_range = Range::new(dst.start + diff, dst.end + diff, 0);
                } else if src.start < dst.start && src.end <= dst.end && src.end >= dst.start {
                    // src overlaps in the left hand side of dst
                    new_range = Range::new(dst.start + diff, src.end + diff, 0);
                } else if src.start >= dst.start && src.end > dst.end && src.start <= dst.end {
                    // src overlaps in the right hand side of dst
                    new_range = Range::new(src.start + diff, dst.end + diff, 0);
                } else {
                    continue;
                }
                result.push(new_range);
            }
        }

        result.sort();

        (result, map.destination_category.clone())
    }

    fn solve(&self, seeds: &[Range<i64>]) -> i64 {
        let mut min_value = i64::MAX;
        let mut current = seeds.to_owned();

        let mut source_category = Category::Seed;

        while source_category != Category::Location {
            (current, source_category) = self.get_next_range(&current, source_category);
        }

        for r in current.iter() {
            min_value = std::cmp::min(min_value, r.start);
        }

        min_value
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let almanac = Almanac::new(input);

    let part1 = almanac.solve(&almanac.seeds_one);
    let part2 = almanac.solve(&almanac.seeds_range);

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
