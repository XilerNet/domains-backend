use lazy_static::lazy_static;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;

use crate::utils::trie::Trie;

const WORDS_FILE: &str = "data/words.txt";
const WORDS_ENV_KEY: &str = "DOMAIN_WORDS_FILE";

lazy_static! {
    static ref WORDS: Trie = {
        info!("Building autocomplete trie");
        let mut builder = Trie::new();
        let words_file = env::var(WORDS_ENV_KEY).unwrap_or_else(|_| WORDS_FILE.to_string());

        debug!("Reading words file: {}", words_file);
        if let Ok(file) = File::open(&words_file) {
            let reader = BufReader::new(file);

            debug!("Inserting words into trie");
            for line in reader.lines() {
                if let Ok(word) = line {
                    builder.insert(word.trim());
                }
            }
        } else {
            error!(
                "Could not open words file {}. (set one using the `{}` env value and make sure it exists)",
                words_file,
                WORDS_ENV_KEY
            );
            warn!("All autocomplete suggestions will be empty due to missing words file.");
        }

        info!("Autocomplete trie built");

        builder
    };
}

pub fn initialize() {
    // Just to trigger the lazy_static
    WORDS.search_prefixes("a", 1);
}

pub fn get_suggestions(search: &str, take: usize) -> Vec<String> {
    WORDS.search_prefixes(search, take)
}
