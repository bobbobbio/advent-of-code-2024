use advent::prelude::*;
use std::fmt;

struct Digit(u64);

impl HasParser for Digit {
    #[into_parser]
    fn parser() -> _ {
        digit().map(|d| Self(d.to_string().parse().unwrap()))
    }
}

#[derive(DisplayMore, PartialEq, Eq)]
enum FsBlock {
    #[display("{_0}")]
    File(u64),
    #[display(".")]
    Empty,
}

#[derive(Default)]
struct Fs(Vec<FsBlock>);

impl Fs {
    fn from_input(input: List<Digit, Nil>) -> Self {
        let mut fs = Self::default();
        let mut id = 0;
        let mut file = true;
        for Digit(d) in input {
            for _ in 0..d {
                if file {
                    fs.0.push(FsBlock::File(id))
                } else {
                    fs.0.push(FsBlock::Empty)
                }
            }
            if file {
                id += 1;
            }
            file = !file;
        }
        fs
    }

    fn checksum(&self) -> u64 {
        self.0
            .iter()
            .enumerate()
            .map(|(i, e)| match e {
                FsBlock::File(id) => i as u64 * id,
                FsBlock::Empty => 0,
            })
            .sum()
    }
}

impl fmt::Display for Fs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.0 {
            write!(f, "{e}")?;
        }
        Ok(())
    }
}

#[part_one]
fn part_one(input: List<Digit, Nil>) -> u64 {
    let mut fs = Fs::from_input(input);

    let mut i = fs.0.len();
    'outer: loop {
        i -= 1;

        if fs.0[i] == FsBlock::Empty {
            continue;
        }

        for j in 0..i {
            if fs.0[j] == FsBlock::Empty {
                fs.0.swap(i, j);
                continue 'outer;
            }
        }

        break;
    }
    fs.checksum()
}

enum Entry {
    File { id: u64, size: u64 },
    Empty { size: u64 },
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File { id, size } => {
                for _ in 0..*size {
                    write!(f, "{id}")?;
                }
            }
            Self::Empty { size } => {
                for _ in 0..*size {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Default)]
struct Fs2(BTreeMap<u64, Entry>);

impl fmt::Display for Fs2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in self.0.values() {
            write!(f, "{e}")?;
        }
        Ok(())
    }
}

impl Fs2 {
    fn from_input(input: List<Digit, Nil>) -> Self {
        let mut fs = Self::default();
        let mut id = 0;
        let mut file = true;
        let mut address = 0;
        for Digit(d) in input {
            if file {
                fs.0.insert(address, Entry::File { id, size: d });
                id += 1;
            } else {
                fs.0.insert(address, Entry::Empty { size: d });
            }
            address += d;
            file = !file;
        }
        fs
    }

    fn find_empty_at_least(&self, file_size: u64) -> Option<u64> {
        for (empty_address, entry) in &self.0 {
            if let Entry::Empty { size: empty_size } = entry {
                if *empty_size >= file_size {
                    return Some(*empty_address);
                }
            }
        }
        None
    }

    fn move_file(&mut self, source: u64, target: u64) {
        let Some(Entry::File {
            id,
            size: file_size,
        }) = self.0.remove(&source)
        else {
            panic!()
        };
        let Some(Entry::Empty { size: empty_size }) = self.0.remove(&target) else {
            panic!()
        };

        self.0.insert(source, Entry::Empty { size: file_size });
        self.0.insert(
            target,
            Entry::File {
                id,
                size: file_size,
            },
        );
        let mut delta = empty_size - file_size;
        if delta > 0 {
            let next_entry = target + empty_size;
            if let Some(Entry::Empty { .. }) = self.0.get(&next_entry) {
                let Some(Entry::Empty { size: next_size }) = self.0.remove(&next_entry) else {
                    panic!()
                };
                delta += next_size;
            }
            self.0
                .insert(target + file_size, Entry::Empty { size: delta });
        }
    }

    fn checksum(&self) -> u64 {
        self.0
            .iter()
            .map(|(a, e)| match e {
                Entry::File { id, size } => (*a..(*a + *size)).map(|a| a * *id).sum(),
                Entry::Empty { .. } => 0,
            })
            .sum()
    }
}

#[part_two]
fn part_two(input: List<Digit, Nil>) -> u64 {
    let mut fs = Fs2::from_input(input);

    let file_addresses: Vec<_> =
        fs.0.iter()
            .filter(|(_, e)| matches!(e, Entry::File { .. }))
            .map(|(k, _)| *k)
            .rev()
            .collect();

    for file_address in file_addresses {
        let Some(Entry::File {
            size: file_size, ..
        }) = fs.0.get(&file_address)
        else {
            panic!()
        };
        if let Some(empty_address) = fs.find_empty_at_least(*file_size) {
            if empty_address < file_address {
                fs.move_file(file_address, empty_address);
            }
        }
    }
    fs.checksum()
}

harness!(part_1: 6399153661894, part_2: 6421724645083);
