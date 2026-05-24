use id_tree::{InsertBehavior, Node, Tree};

use crate::file_system_entry::{Directory, FileSystemEntry};

use self::interpreter::parse_commands;

mod file_system_entry;
mod interpreter;
mod tree;

fn main() {
    let input = include_str!("input.txt");

    let a1 = p1(input);
    println!("a1 = {a1}");

    let a2 = p2(input);
    println!("a2 = {a2}");
}

///
/// Both problems start the same:
///
/// 1. Parse the commands
/// 2. Interpret the commands and build the File tree
/// 3. Calculate the size of the directories
///
/// This could be significantly cleaned up and made more readable, but I don't care right now
///
fn setup(input: &str) -> Vec<(Directory, usize)> {
    // Parse commands
    let (_, commands) = parse_commands(input).unwrap();

    // Init tree
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

    // Interpret the commands and build the tree
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

    // tree::pretty_print(&tree);

    // Calculate dir sizes
    let root = tree.get(tree.root_node_id().unwrap()).unwrap();
    let mut dir_sizes = vec![];
    tree::calculate_dir_sizes(&tree, root, &mut dir_sizes);

    // println!("{dir_sizes:#?}");

    dir_sizes
}

/// find all of the directories with a total size of at most 100000, then calculate the sum of
/// their total sizes. You can count directories more than once
fn p1(input: &str) -> usize {
    let dir_sizes = setup(input);

    dir_sizes
        .iter()
        .map(|x| x.1)
        .filter(|size| *size < 100_000)
        .sum()
}

/// Given 70000000 of disk space and targeting at least 30000000 of unused space,
/// find the smallest directory to remove that would make the disk have that amount of space
/// available. Return the size of that directory
fn p2(input: &str) -> usize {
    let dir_sizes = setup(input);

    let disk_size: usize = 70_000_000;
    let target_free_space: usize = 30_000_000;

    // Disk used is the size of the root folder, which is the max value
    let disk_used = dir_sizes.iter().max_by_key(|d| d.1).unwrap().1;
    let current_free_space = disk_size - disk_used;
    let need_to_free_at_least = target_free_space - current_free_space;

    // println!("{dir_sizes:#?}");
    // println!("root = {disk_used}");
    // println!("available = {current_free_space}");
    // println!("need_to_free_at_least = {need_to_free_at_least}");

    dir_sizes
        .iter()
        .map(|x| x.1)
        .filter(|size| *size > need_to_free_at_least)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        indoc::indoc! {"
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
        "}
    }

    #[test]
    fn p1_example() {
        assert_eq!(p1(input()), 95_437);
    }

    #[test]
    fn p2_example() {
        assert_eq!(p2(input()), 24_933_642);
    }
}
