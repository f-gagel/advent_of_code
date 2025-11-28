use std::convert::Infallible;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;
use common::input::Input;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from]std::io::Error),
    #[error("Duplicate ls command for directory")]
    DuplicateLs,
    #[error("Unknown command \"{0}\"")]
    UnknownCommand(String),
    #[error("Missing separator '{0}'")]
    MissingSeparator(char),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError)
}

#[derive(Debug, Eq, PartialEq)]
enum Change<'a> {
    Backtrack,
    Enter(&'a str),
}

#[derive(Debug, Eq, PartialEq)]
enum Command<'a> {
    Cd(Change<'a>),
    Ls,
}

fn try_parse_command(s: &str) -> Option<Result<Command<'_>, Error>> {
    if &s[..2] != "$ " {
        return None;
    }

    let (cmd, p) = s[2..].split_at(2);
    let command = match cmd {
        "cd" => {
            let p = p.trim();
            if p == ".." {
                Command::Cd(Change::Backtrack)
            } else {
                Command::Cd(Change::Enter(p.trim()))
            }
        }
        "ls" => Command::Ls,
        _ => return Some(Err(Error::UnknownCommand(cmd.to_string()))),
    };
    Some(Ok(command))
}

fn parse_dir(name: String, reader: &mut impl BufRead, buf: &mut String) -> Result<Directory, Error> {
    let mut dir = Directory {
        name,
        directories: Vec::new(),
        files: Vec::new(),
    };

    buf.clear();
    reader.read_line(buf)?;
    try_parse_command(buf);

    loop {
        buf.clear();
        if reader.read_line(buf)? == 0 {
            break;
        }
        if let Some(cmd) = try_parse_command(buf) {
            match cmd? {
                Command::Cd(cd) => match cd {
                    Change::Backtrack => break,
                    Change::Enter(n) => {
                        dir.directories.push(parse_dir(n.to_owned(), reader, buf)?);
                    }
                },
                Command::Ls => return Err(Error::DuplicateLs),
            }
        } else if buf.chars().nth(0).unwrap().is_numeric() {
            let i = buf.find(' ').ok_or(Error::MissingSeparator(' '))?;
            let (size, _name) = buf.split_at(i);
            let size = u64::from_str(size)?;
            dir.files.push(File { size })
        }
    }
    Ok(dir)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl Input<'_> for Directory {
    type Error = Error;
    fn parse<R: BufRead>(mut read: R) -> Result<Self, Self::Error> {
        let mut buf = String::with_capacity(256);
        read.read_line(&mut buf)?;
        buf.clear();
        parse_dir("/".to_string(), &mut read, &mut buf)
    }
}

impl Directory {
    fn get_total_size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum::<u64>()
            + self
            .directories
            .iter()
            .map(Directory::get_total_size)
            .sum::<u64>()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct File {
    size: u64,
}

fn traverse<'a>(root: &'a Directory, f: &mut impl FnMut(&Directory)) {
    f(root);
    root.directories.iter().for_each(|d| traverse(d, f));
}

pub fn task1(input: Directory) -> Result<u64, Infallible>{
    let mut total_size = 0;
    traverse(&input, &mut |dir| {
        let size = dir.get_total_size();
        if size <= 100_000 {
            total_size += size;
        }
    });

    Ok(total_size)
}

pub fn task2(input: Directory)-> Result<u64, Infallible> {
    const TOTAL_SPACE: u64 = 70_000_000;
    const REQURIED_SPACE: u64 = 30_000_000;

    let occupied_space = input.get_total_size();
    let must_free = occupied_space + REQURIED_SPACE - TOTAL_SPACE;

    let mut smallest_free = u64::MAX;
    traverse(&input, &mut |dir| {
        let size = dir.get_total_size();
        if size >= must_free && size < smallest_free {
            smallest_free = size;
        }
    });

    Ok(smallest_free)
}
