use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{multispace1, newline};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::IResult;

use crate::file_system_entry::{Directory, File, FileSystemEntry};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Cd(Directory),
    CdParent,
    CdRoot,
    Ls(Vec<FileSystemEntry>),
}

pub fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    many0(alt((parse_cd, parse_ls)))(input)
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    map(
        delimited(tag("$ cd "), parse_path, newline),
        |name| match name {
            "/" => Command::CdRoot,
            ".." => Command::CdParent,
            _ => Command::Cd(Directory { name: name.into() }),
        },
    )(input)
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    map(
        preceded(tag("$ ls\n"), many0(parse_file_system_entry_line)),
        Command::Ls,
    )(input)
}

fn parse_file_system_entry_line(input: &str) -> IResult<&str, FileSystemEntry> {
    terminated(alt((parse_dir, parse_file)), newline)(input)
}

fn parse_path(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c))(input)
}

fn parse_dir(input: &str) -> IResult<&str, FileSystemEntry> {
    map(preceded(tag("dir "), parse_path), |name: &str| {
        FileSystemEntry::Directory(Directory { name: name.into() })
    })(input)
}

fn parse_file(input: &str) -> IResult<&str, FileSystemEntry> {
    map(
        separated_pair(nom::character::complete::u64, multispace1, parse_path),
        |(size, name)| {
            FileSystemEntry::File(File {
                name: name.into(),
                size: size.try_into().unwrap(),
            })
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cd_test() {
        assert_eq!(
            parse_cd("$ cd a\n"),
            Ok(("", Command::Cd(Directory { name: "a".into() })))
        );

        assert_eq!(parse_cd("$ cd /\n"), Ok(("", Command::CdRoot)));
        assert_eq!(parse_cd("$ cd ..\n"), Ok(("", Command::CdParent)));
    }

    #[test]
    fn parse_dir_test() {
        assert_eq!(
            parse_dir("dir asdf"),
            Ok((
                "",
                FileSystemEntry::Directory(Directory {
                    name: "asdf".into()
                })
            ))
        );
    }

    #[test]
    fn parse_file_test() {
        assert_eq!(
            parse_file("2557 g"),
            Ok((
                "",
                FileSystemEntry::File(File {
                    name: "g".into(),
                    size: 2557
                })
            ))
        );

        assert_eq!(
            parse_file("62596 h.lst"),
            Ok((
                "",
                FileSystemEntry::File(File {
                    name: "h.lst".into(),
                    size: 62596
                })
            ))
        );
    }

    #[test]
    fn parse_ls_test() {
        let input = indoc::indoc! {"
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd a
        "};

        assert_eq!(
            parse_ls(input),
            Ok((
                "$ cd a\n",
                Command::Ls(vec![
                    FileSystemEntry::Directory(Directory { name: "e".into() }),
                    FileSystemEntry::File(File {
                        name: "f".into(),
                        size: 29116
                    }),
                    FileSystemEntry::File(File {
                        name: "g".into(),
                        size: 2557
                    }),
                    FileSystemEntry::File(File {
                        name: "h.lst".into(),
                        size: 62596
                    }),
                ])
            ))
        );
    }

    #[test]
    fn parse_commands_test() {
        let input = indoc::indoc! {"
            $ cd /
            $ ls
            dir a
        "};

        assert_eq!(
            parse_commands(input),
            Ok((
                "",
                vec![
                    Command::CdRoot,
                    Command::Ls(vec![FileSystemEntry::Directory(Directory {
                        name: "a".into()
                    }),])
                ]
            ))
        );
    }

    #[test]
    fn smoke_test() {
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

        assert_eq!(
            parse_commands(input),
            Ok((
                "",
                vec![
                    Command::CdRoot,
                    Command::Ls(vec![
                        FileSystemEntry::Directory(Directory { name: "a".into() }),
                        FileSystemEntry::File(File {
                            name: "b.txt".into(),
                            size: 14_848_514
                        }),
                        FileSystemEntry::File(File {
                            name: "c.dat".into(),
                            size: 8_504_156
                        }),
                        FileSystemEntry::Directory(Directory { name: "d".into() }),
                    ]),
                    Command::Cd(Directory { name: "a".into() }),
                    Command::Ls(vec![
                        FileSystemEntry::Directory(Directory { name: "e".into() }),
                        FileSystemEntry::File(File {
                            name: "f".into(),
                            size: 29116
                        }),
                        FileSystemEntry::File(File {
                            name: "g".into(),
                            size: 2557
                        }),
                        FileSystemEntry::File(File {
                            name: "h.lst".into(),
                            size: 62596
                        }),
                    ]),
                    Command::Cd(Directory { name: "e".into() }),
                    Command::Ls(vec![FileSystemEntry::File(File {
                        name: "i".into(),
                        size: 584
                    }),]),
                    Command::CdParent,
                    Command::CdParent,
                    Command::Cd(Directory { name: "d".into() }),
                    Command::Ls(vec![
                        FileSystemEntry::File(File {
                            name: "j".into(),
                            size: 4_060_174
                        }),
                        FileSystemEntry::File(File {
                            name: "d.log".into(),
                            size: 8_033_020
                        }),
                        FileSystemEntry::File(File {
                            name: "d.ext".into(),
                            size: 5_626_152
                        }),
                        FileSystemEntry::File(File {
                            name: "k".into(),
                            size: 7_214_296
                        }),
                    ]),
                ]
            ))
        );
    }
}
