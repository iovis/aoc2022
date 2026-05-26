use color_eyre::Result;
use day3::{fast, slow};

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let input = include_str!("../input.txt");

    // println!("slow p1 = {:?}", slow::p1(input));
    println!("fast p1 = {:?}", fast::p1(input));
    // println!("slow p2 = {:?}", slow::p2(input));
    // println!("fast p2 = {:?}", fast::p2(input));

    Ok(())
}
