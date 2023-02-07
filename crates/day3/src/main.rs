use color_eyre::Result;
use day3::{fast, slow};

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let input = include_str!("input.txt");

    let a1 = slow::p1(input);
    println!("a1: {a1:?}");

    let a1 = fast::p1(input);
    println!("a1: {a1:?}");

    let a2 = slow::p2(input);
    println!("a2: {a2:?}");

    let a2 = fast::p2(input);
    println!("a2: {a2:?}");

    Ok(())
}
