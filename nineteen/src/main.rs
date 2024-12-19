use advent::prelude::*;
use enum_map::{Enum, EnumMap};

#[derive(Copy, Clone, Debug, HasParser, Enum)]
enum Color {
    #[parse(string = "w")]
    White,
    #[parse(string = "u")]
    Blue,
    #[parse(string = "b")]
    Black,
    #[parse(string = "r")]
    Red,
    #[parse(string = "g")]
    Green,
}

#[derive(Debug, HasParser)]
struct Towel(Vec<Color>);

#[derive(Debug, HasParser)]
struct Pattern(Vec<Color>);

#[derive(Debug, HasParser)]
#[parse(sep_by = "\n\n")]
struct Input {
    towels: List<Towel, SepBy<CommaSpace>>,
    patterns: List<Pattern, TermWith<NewLine>>,
}

#[derive(Default, Debug)]
struct ColorTrieNode {
    next: EnumMap<Color, Option<Box<ColorTrieNode>>>,
    end: bool,
}

impl ColorTrieNode {
    fn add(&mut self, towel: &Towel) {
        let mut node = self;
        for c in &towel.0 {
            let next = &mut node.next[*c];
            if let Some(next) = next {
                node = &mut *next;
            } else {
                *next = Some(Box::new(Self::default()));
                node = next.as_mut().unwrap();
            }
        }
        node.end = true;
    }

    fn look_up(&self, c: Color) -> Option<&Self> {
        self.next[c].as_ref().map(|b| b.as_ref())
    }
}

#[derive(Default)]
struct Cache(HashMap<usize, bool>);

fn is_possible(cache: &mut Cache, trie: &ColorTrieNode, index: usize, pattern: &[Color]) -> bool {
    if let Some(res) = cache.0.get(&index) {
        return *res;
    }

    if index >= pattern.len() {
        cache.0.insert(index, true);
        true
    } else {
        let mut node = trie;
        for i in index..pattern.len() {
            let Some(next) = node.look_up(pattern[i]) else {
                cache.0.insert(index, false);
                return false;
            };
            if next.end && is_possible(cache, trie, i + 1, pattern) {
                cache.0.insert(index, true);
                return true;
            }
            node = next;
        }
        cache.0.insert(index, false);
        false
    }
}

#[part_one]
fn part_one(input: Input) -> usize {
    let mut trie = ColorTrieNode::default();
    for towel in &input.towels {
        trie.add(towel);
    }
    let mut total = 0;
    for pat in &input.patterns {
        let mut cache = Default::default();
        if is_possible(&mut cache, &trie, 0, &pat.0) {
            total += 1;
        }
    }
    total
}

#[derive(Default)]
struct Cache2(HashMap<usize, usize>);

fn num_ways(cache: &mut Cache2, trie: &ColorTrieNode, index: usize, pattern: &[Color]) -> usize {
    if let Some(res) = cache.0.get(&index) {
        return *res;
    }

    if index >= pattern.len() {
        cache.0.insert(index, 1);
        1
    } else {
        let mut node = trie;
        let mut total = 0;
        for i in index..pattern.len() {
            let Some(next) = node.look_up(pattern[i]) else {
                break;
            };
            if next.end {
                total += num_ways(cache, trie, i + 1, pattern);
            }
            node = next;
        }
        cache.0.insert(index, total);
        total
    }
}

#[part_two]
fn part_two(input: Input) -> usize {
    let mut trie = ColorTrieNode::default();
    for towel in &input.towels {
        trie.add(towel);
    }
    let mut total = 0;
    for pat in &input.patterns {
        let mut cache = Default::default();
        total += num_ways(&mut cache, &trie, 0, &pat.0);
    }
    total
}

harness!(part_1: 206, part_2: 622121814629343);
