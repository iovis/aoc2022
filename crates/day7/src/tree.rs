use id_tree::{Node, Tree};

use crate::file_system_entry::{Directory, FileSystemEntry};

pub fn pretty_print<T: std::fmt::Debug>(tree: &Tree<T>) {
    let mut s = String::new();
    tree.write_formatted(&mut s).unwrap();
    println!("{s}");
}

pub fn calculate_dir_sizes<'a>(
    tree: &'a Tree<FileSystemEntry>,
    node: &'a Node<FileSystemEntry>,
    dirs: &mut Vec<(&'a Directory, usize)>, // Output of the calculation. I know, not the cleanest
) -> usize {
    let mut size: usize = 0;
    let entry = node.data();

    match entry {
        FileSystemEntry::File(f) => size += f.size,
        FileSystemEntry::Directory(dir) => {
            for child_id in node.children() {
                let child_node = tree.get(child_id).unwrap();
                size += calculate_dir_sizes(tree, child_node, dirs);
            }

            dirs.push((dir, size));
        }
    }

    // println!("{entry:?} => {size}");

    size
}
