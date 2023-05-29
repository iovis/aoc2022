use id_tree::{Node, Tree};

use crate::file_system_entry::{Directory, FileSystemEntry};

use self::interpreter::parse_commands;

mod file_system_entry;
mod interpreter;

fn main() {
    let input = include_str!("input.txt");

    let a1 = p1(input);
    println!("a1 = {a1}");
}

fn p1(input: &str) -> usize {
    let (_, commands) = parse_commands(input).unwrap();

    dbg!(commands);

    let mut tree = Tree::new();

    tree.insert(
        Node::new(FileSystemEntry::Directory(Directory {
            name: "/".to_string(),
        })),
        id_tree::InsertBehavior::AsRoot,
    )
    .unwrap();

    let current_node = tree.root_node_id().unwrap();

    dbg!(tree);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod p1 {
        use super::*;

        #[test]
        fn p1_example() {
            let input = indoc::indoc! {"
                $ cd /
                $ ls
                dir a
                14848514 b.txt
                8504156 c.dat
                dir d
                $ cd a
                $ ls
                dir e
                29116 f
                2557 g
                62596 h.lst
                $ cd e
                $ ls
                584 i
                $ cd ..
                $ cd ..
                $ cd d
                $ ls
                4060174 j
                8033020 d.log
                5626152 d.ext
                7214296 k
            "};

            assert_eq!(p1(input), 95437);
        }
    }
}
