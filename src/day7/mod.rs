use crate::Solution;
use std::{cell::Cell, collections::HashMap, error::Error};

struct Filesystem<'a> {
    directories: Vec<Directory<'a>>,
}

impl<'a> Filesystem<'a> {
    fn parse(input: &'a str) -> Result<Self, Box<dyn Error>> {
        let mut fs = Self {
            directories: vec![Directory::new(None)],
        };
        let mut current_directory = !0;
        for line in input.lines() {
            if line == "$ cd /" {
                current_directory = 0;
            } else if line == "$ cd .." {
                current_directory = fs.directories[current_directory]
                    .parent_dir
                    .ok_or("Cannot use cd .. for root directory")?;
            } else if line == "$ ls" {
                // Do nothing
            } else if let Some(path) = line.strip_prefix("$ cd ") {
                match fs.directories[current_directory]
                    .files
                    .get(path)
                    .ok_or("Unrecognized directory")?
                {
                    &Kind::Directory { id } => current_directory = id,
                    _ => return Err("Expected a directory".into()),
                }
            } else if let Some(path) = line.strip_prefix("dir ") {
                let id = fs.directories.len();
                fs.directories.push(Directory::new(Some(current_directory)));
                fs.directories[current_directory]
                    .files
                    .insert(path, Kind::Directory { id });
            } else {
                let (size, name) = line.split_once(' ').ok_or("Expected a valid line")?;
                fs.directories[current_directory].files.insert(
                    name,
                    Kind::File {
                        size: size.parse()?,
                    },
                );
            }
        }
        fs.compute_total_size(0);
        Ok(fs)
    }

    fn compute_total_size(&self, index: usize) -> u32 {
        let directory = &self.directories[index];
        let sum = directory
            .files
            .iter()
            .map(|(_, kind)| match *kind {
                Kind::Directory { id } => self.compute_total_size(id),
                Kind::File { size } => size,
            })
            .sum();
        directory.total_size.set(sum);
        sum
    }
}

struct Directory<'a> {
    parent_dir: Option<usize>,
    files: HashMap<&'a str, Kind>,
    total_size: Cell<u32>,
}

impl Directory<'_> {
    fn new(parent_dir: Option<usize>) -> Self {
        Self {
            parent_dir,
            files: HashMap::new(),
            total_size: Cell::new(0),
        }
    }
}

enum Kind {
    File { size: u32 },
    Directory { id: usize },
}

pub(super) const DAY7: Solution = Solution {
    part1: |input| {
        let fs = Filesystem::parse(input)?;
        Ok(fs
            .directories
            .iter()
            .map(|dir| dir.total_size.get())
            .filter(|&total_size| total_size <= 100_000)
            .sum::<u32>()
            .to_string())
    },
    part2: |input| {
        let fs = Filesystem::parse(input)?;
        let necessary_to_free = 30_000_000_u32
            .checked_sub(
                70_000_000_u32
                    .checked_sub(fs.directories[0].total_size.get())
                    .ok_or("Root directory larger than total available space")?,
            )
            .ok_or("No directory needs to be deleted")?;
        Ok(fs
            .directories
            .iter()
            .map(|dir| dir.total_size.get())
            .filter(|&total_size| total_size >= necessary_to_free)
            .min()
            .ok_or("Couldn't find a small enough directory")?
            .to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "$ cd /"
        "$ ls"
        "dir a"
        "14848514 b.txt"
        "8504156 c.dat"
        "dir d"
        "$ cd a"
        "$ ls"
        "dir e"
        "29116 f"
        "2557 g"
        "62596 h.lst"
        "$ cd e"
        "$ ls"
        "584 i"
        "$ cd .."
        "$ cd .."
        "$ cd d"
        "$ ls"
        "4060174 j"
        "8033020 d.log"
        "5626152 d.ext"
        "7214296 k"
    );
    test!(
        DAY7.part1,
        example: EXAMPLE => 95437,
        input: 1334506,
    );
    test!(
        DAY7.part2,
        example: EXAMPLE => 24933642,
        input: 7421137,
    );
}
