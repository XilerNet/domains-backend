use lazy_static::lazy_static;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::utils::trie::Trie;

lazy_static! {
    static ref WORDS: Trie = {
        let mut builder = Trie::new();
        println!("Building trie...");
        if let Ok(file) = File::open("data/words.txt") {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(word) = line {
                    builder.insert(word.trim());
                }
            }
        }

        builder
    };
}

pub fn get_suggestions(search: &str, take: usize) -> Vec<String> {
    WORDS.search_prefixes(search, take)
}
