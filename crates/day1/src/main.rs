use color_eyre::Result;
use itertools::Itertools;

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let input = include_str!("day1.txt");
    let lines = input.lines().collect_vec();

    let groups = lines.split(|line| line.trim().is_empty()).collect_vec();
    // println!("{groups:?}");

    let answer: usize = groups
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!("Answer: {answer}");

    Ok(())
}
