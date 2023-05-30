fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
}

#[derive(Debug, Default)]
struct Tree {
    height: u32,
    visible: bool,
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vis = if self.visible { 'v' } else { 'n' };
        write!(f, "{}{vis}", self.height)
    }
}

/// Input is a grid of numbers representing tree's height (0-9)
/// A tree is visible if all trees in a direction are strictly smaller
/// By definition, all trees in the edge are visible
/// Only consider cross directions, not diagonal
///
/// How many trees are visible?
fn p1(input: &str) -> usize {
    // parse input into an actual grid
    let mut grid: Vec<Vec<Tree>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|height| Tree {
                    height,
                    ..Default::default()
                })
                .collect()
        })
        .collect();

    let length = grid.len();

    #[allow(clippy::needless_range_loop)] // I do feel a bit dirty
    for i in 0..length {
        for j in 0..length {
            check_visibility_of(&mut grid, i, j);
        }
    }

    // print_grid(&grid);

    grid.iter().flatten().filter(|tree| tree.visible).count()
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn check_visibility_of(grid: &mut Vec<Vec<Tree>>, row: usize, col: usize) {
    let length = grid.len();
    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    for direction in directions {
        if grid[row][col].visible {
            return;
        }

        match direction {
            Direction::North => {
                let mut visible = true;

                if row != 0 {
                    for i in (0..row).rev() {
                        if grid[i][col].height >= grid[row][col].height {
                            visible = false;
                            break;
                        }
                    }
                }

                grid[row].get_mut(col).unwrap().visible = visible;
            }
            Direction::East => {
                let mut visible = true;

                if col != length - 1 {
                    for j in col + 1..length {
                        if grid[row][j].height >= grid[row][col].height {
                            visible = false;
                            break;
                        }
                    }
                }

                grid[row].get_mut(col).unwrap().visible = visible;
            }
            Direction::South => {
                let mut visible = true;

                if row != length - 1 {
                    for i in row + 1..length {
                        if grid[i][col].height >= grid[row][col].height {
                            visible = false;
                            break;
                        }
                    }
                }

                grid[row].get_mut(col).unwrap().visible = visible;
            }
            Direction::West => {
                let mut visible = true;

                if col != 0 {
                    for j in (0..col).rev() {
                        if grid[row][j].height >= grid[row][col].height {
                            visible = false;
                            break;
                        }
                    }
                }

                grid[row].get_mut(col).unwrap().visible = visible;
            }
        }
    }
}

#[allow(unused)]
fn print_grid(grid: &Vec<Vec<Tree>>) {
    let length = grid.len();

    #[allow(clippy::needless_range_loop)]
    for i in 0..length {
        for j in 0..length {
            print!("{} ", grid[i][j]);
        }

        println!("\n");
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc! {"
            30373
            25512
            65332
            33549
            35390
        "};

        assert_eq!(p1(input), 21);
    }
}
