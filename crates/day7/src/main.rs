use id_tree::{InsertBehavior, Node, Tree};

use crate::file_system_entry::{Directory, FileSystemEntry};

use self::interpreter::parse_commands;

mod file_system_entry;
mod interpreter;

fn main() {
    let input = include_str!("input.txt");

    let a1 = p1(input);
    println!("a1 = {a1}");
}

/// find all of the directories with a total size of at most 100000, then calculate the sum of
/// their total sizes. You can count directories more than once
fn p1(input: &str) -> usize {
    let (_, commands) = parse_commands(input).unwrap();

    let mut tree = Tree::new();
    let root_id = tree
        .insert(
            Node::new(FileSystemEntry::Directory(Directory {
                name: "/".to_string(),
            })),
            InsertBehavior::AsRoot,
        )
        .unwrap();

    let mut current_node_id = root_id;

    for command in commands {
        match command {
            interpreter::Command::CdRoot => current_node_id = tree.root_node_id().unwrap().clone(),
            interpreter::Command::CdParent => {
                current_node_id = tree
                    .get(&current_node_id)
                    .unwrap()
                    .parent()
                    .unwrap()
                    .clone();
            }
            interpreter::Command::Cd(new_dir) => {
                // Find new directory from the children of the current one
                current_node_id = tree
                    .children_ids(&current_node_id)
                    .unwrap()
                    .filter_map(|node_id| {
                        let node = tree.get(node_id).unwrap();

                        match node.data() {
                            FileSystemEntry::Directory(dir) => Some((node_id, dir)),
                            FileSystemEntry::File(_) => None,
                        }
                    })
                    .find(|(_node_id, dir)| *dir == &new_dir)
                    .map(|(node_id, _)| node_id)
                    .unwrap()
                    .clone();
            }
            interpreter::Command::Ls(entries) => {
                for entry in entries {
                    tree.insert(
                        Node::new(entry),
                        InsertBehavior::UnderNode(&current_node_id),
                    )
                    .unwrap();
                }
            }
        }
    }

    print_tree(&tree);

    todo!()
}

fn print_tree<T: std::fmt::Debug>(tree: &Tree<T>) {
    let mut s = String::new();
    tree.write_formatted(&mut s).unwrap();
    println!("{s}");
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
