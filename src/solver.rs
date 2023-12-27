use color_eyre::eyre::Result;
use tokio::{fs::File, io::AsyncReadExt};
use tracing::info;

#[derive(Debug)]
pub struct Solver {
    input: String,
    day: i32,
    answer: Option<Answer>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Answer {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

impl Default for Answer {
    fn default() -> Self {
        Self {
            part1: Some("0".to_string()),
            part2: Some("0".to_string()),
        }
    }
}

impl Solver {
    pub async fn new(day: i32) -> Result<Self> {
        let path = format!("input/{:0>2}", day);
        let mut file = File::open(path).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;

        Ok(Self {
            input: content,
            day,
            answer: None,
        })
    }

    pub fn print_answer(&self) {
        let p1 = self.answer.as_ref().unwrap().part1.as_ref().unwrap();
        let p2 = self.answer.as_ref().unwrap().part2.as_ref().unwrap();
        info!("Day {:0>2} part 1: {}", self.day, p1);
        info!("Day {:0>2} part 2: {}", self.day, p2);
    }

    pub async fn solve(&mut self) -> Result<()> {
        let answer = match self.day {
            1 => crate::day01::solve(&self.input)?,
            2 => crate::day02::solve(&self.input)?,
            3 => crate::day03::solve(&self.input)?,
            4 => crate::day04::solve(&self.input)?,
            5 => crate::day05::solve(&self.input)?,
            6 => crate::day06::solve(&self.input)?,
            7 => crate::day07::solve(&self.input)?,
            8 => crate::day08::solve(&self.input)?,
            9 => crate::day09::solve(&self.input)?,
            10 => crate::day10::solve(&self.input)?,
            11 => crate::day11::solve(&self.input)?,
            12 => crate::day12::solve(&self.input)?,
            13 => crate::day13::solve(&self.input)?,
            14 => crate::day14::solve(&self.input)?,
            15 => crate::day15::solve(&self.input)?,
            16 => crate::day16::solve(&self.input)?,
            _ => todo!(),
        };

        self.answer = Some(answer);

        Ok(())
    }
}
