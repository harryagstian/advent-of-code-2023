use color_eyre::eyre::Result;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug)]
pub struct Solver {
    input: String,
    day: i32,
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
        })
    }

    pub async fn solve(&self) -> Result<Answer> {
        let answer = match self.day {
            1 => crate::day_01::solve_day01(&self.input).await?,
            _ => todo!(),
        };

        Ok(answer)
    }
}
