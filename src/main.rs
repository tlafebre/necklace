// https://www.reddit.com/r/dailyprogrammer/comments/ffxabb/20200309_challenge_383_easy_necklace_matching/

use std::collections::VecDeque;

#[derive(Debug)]
struct Necklace {
    name: String,
    letters: VecDeque<char>,
    size: usize,
    index: usize,
}

impl Necklace {
    fn new(name: String) -> Necklace {
        let chars: Vec<char> = name.chars().collect();
        let size: usize = name.len();
        let letters = VecDeque::from(chars);

        Necklace {
            name,
            letters,
            size,
            index: 0,
        }
    }

    fn same_as(self, other: Necklace) -> bool {
        for each in self {
            if other.name == each {
                return true;
            }
        }
        false
    }
}

impl Iterator for Necklace {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return Some("".to_string());
        }
        if self.index < self.size {
            let first = self.letters.pop_front();
            self.letters.push_back(first.unwrap());
            self.index += 1;

            let new_name: String = self.letters.iter().collect();

            return Some(new_name);
        }
        None
    }
}

fn same_necklace(name1: &str, name2: &str) -> bool {
    let necklace1 = Necklace::new(name1.to_string());
    let necklace2 = Necklace::new(name2.to_string());

    necklace1.same_as(necklace2)
}

fn main() {
    assert_eq!(same_necklace("nicole", "icolen"), true);
    assert_eq!(same_necklace("nicole", "lenico"), true);
    assert_eq!(same_necklace("nicole", "coneli"), false);
    assert_eq!(same_necklace("aabaaaaabaab", "aabaabaabaaa"), true);
    assert_eq!(same_necklace("abc", "cba"), false);
    assert_eq!(same_necklace("xxyyy", "xxxyy"), false);
    assert_eq!(same_necklace("xyxxz", "xxyxz"), false);
    assert_eq!(same_necklace("x", "x"), true);
    assert_eq!(same_necklace("x", "xx"), false);
    assert_eq!(same_necklace("x", ""), false);
    assert_eq!(same_necklace("", ""), true);
}
