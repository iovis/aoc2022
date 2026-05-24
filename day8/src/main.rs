fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

#[derive(Debug, Default)]
struct Tree {
    height: u32,
    visible: bool,
    score: u32,
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vis = if self.visible { 'v' } else { 'n' };
        write!(f, "{}{vis}{:02}", self.height, self.score)
    }
}

// TODO: Maybe try rewriting to one-dimensional array with custom indexing
type Grid = Vec<Vec<Tree>>;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|height| Tree {
                    height,
                    score: 1,
                    ..Default::default()
                })
                .collect()
        })
        .collect()
}

/// Input is a grid of numbers representing tree's height (0-9)
/// A tree is visible if all trees in a direction are strictly smaller
/// By definition, all trees in the edge are visible
/// Only consider cross directions, not diagonal
///
/// How many trees are visible?
fn p1(input: &str) -> usize {
    // parse input into an actual grid
    let mut grid: Grid = parse_grid(input);
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

fn check_visibility_of(grid: &mut Grid, row: usize, col: usize) {
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

/// Calculate the best scenic score
///
/// scenic score: multiplying together its viewing distance in all four directions
fn p2(input: &str) -> u32 {
    // parse input into an actual grid
    let mut grid: Grid = parse_grid(input);
    let length = grid.len();

    #[allow(clippy::needless_range_loop)] // I do feel a bit dirty
    for i in 0..length {
        for j in 0..length {
            calculate_score_of(&mut grid, i, j);
        }
    }

    // print_grid(&grid);

    grid.iter().flatten().map(|tree| tree.score).max().unwrap()
}

fn calculate_score_of(grid: &mut Grid, row: usize, col: usize) {
    let length = grid.len();
    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    for direction in directions {
        if grid[row][col].score == 0 {
            return;
        }

        match direction {
            Direction::North => {
                let mut score = 0;

                if row != 0 {
                    for i in (0..row).rev() {
                        score += 1;

                        if grid[i][col].height >= grid[row][col].height {
                            break;
                        }
                    }
                }

                let tree = grid[row].get_mut(col).unwrap();
                tree.score *= score;
            }
            Direction::East => {
                let mut score = 0;

                if col != length - 1 {
                    for j in col + 1..length {
                        score += 1;

                        if grid[row][j].height >= grid[row][col].height {
                            break;
                        }
                    }
                }

                let tree = grid[row].get_mut(col).unwrap();
                tree.score *= score;
            }
            Direction::South => {
                let mut score = 0;

                if row != length - 1 {
                    for i in row + 1..length {
                        score += 1;

                        if grid[i][col].height >= grid[row][col].height {
                            break;
                        }
                    }
                }

                let tree = grid[row].get_mut(col).unwrap();
                tree.score *= score;
            }
            Direction::West => {
                let mut score = 0;

                if col != 0 {
                    for j in (0..col).rev() {
                        score += 1;

                        if grid[row][j].height >= grid[row][col].height {
                            break;
                        }
                    }
                }

                let tree = grid[row].get_mut(col).unwrap();
                tree.score *= score;
            }
        }
    }
}

#[allow(unused)]
fn print_grid(grid: &Grid) {
    let length = grid.len();

    #[allow(clippy::needless_range_loop)]
    for i in 0..length {
        for j in 0..length {
            print!("{} ", grid[i][j]);
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    fn input() -> &'static str {
        indoc! {"
            30373
            25512
            65332
            33549
            35390
        "}
    }

    #[test]
    fn p1_test() {
        assert_eq!(p1(input()), 21);
    }

    #[test]
    fn p2_test() {
        assert_eq!(p2(input()), 8);
    }
}
