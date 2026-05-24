use itertools::Itertools;

fn main() {
    // tracing_subscriber::fmt::init();

    let input = include_str!("../input.txt");

    // println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

fn p1(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let groups = lines.split(|line| line.trim().is_empty()).collect_vec();

    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sorted()
        .next_back()
        .unwrap()
}

fn p2(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let groups = lines.split(|line| line.trim().is_empty()).collect_vec();

    groups
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
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn p1_test() {
        let _ = tracing_subscriber::fmt::try_init();
        let input = indoc::indoc! {"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "};

        assert_eq!(p1(input), 24_000);
    }

    #[test]
    fn p2_test() {
        let _ = tracing_subscriber::fmt::try_init();
        let input = indoc::indoc! {"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "};

        assert_eq!(p2(input), 45_000);
    }
}
