#[derive(PartialEq, Eq)]
pub enum FileSystemEntry {
    Directory(Directory),
    File(File),
}

impl std::fmt::Debug for FileSystemEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Directory(dir) => write!(f, "{} (dir)", dir.name),
            Self::File(file) => write!(f, "{} (file, size={})", file.name, file.size),
        }
    }
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
