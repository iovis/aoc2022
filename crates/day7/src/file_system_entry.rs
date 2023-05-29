#[derive(Debug, PartialEq, Eq)]
pub enum FileSystemEntry {
    Directory(Directory),
    File(File),
}

// TODO: Use references
#[derive(Debug, PartialEq, Eq)]
pub struct Directory {
    pub name: String,
    // size: usize?
}

#[derive(Debug, PartialEq, Eq)]
pub struct File {
    pub name: String,
    pub size: usize,
}
