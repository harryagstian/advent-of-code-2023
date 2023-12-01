use color_eyre::eyre::Result;
mod day_01;
mod solver;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let day1 = solver::Solver::new(1).await?;
    let r = day1.solve().await?;
    dbg!(r);

    Ok(())
}
