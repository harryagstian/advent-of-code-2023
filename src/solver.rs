use color_eyre::eyre::Result;
use tokio::{fs::File, io::AsyncReadExt};
use tracing::info;

#[derive(Debug)]
pub struct Solver {
    input: String,
    day: i32,
    answer: Option<Answer>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Answer {
    pub part1: Option<String>,
    pub part2: Option<String>,
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
            1 => crate::day01::solve_day01(&self.input).await?,
            _ => todo!(),
        };

        self.answer = Some(answer);

        Ok(())
    }
}
