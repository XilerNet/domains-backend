#[derive(Default)]
struct TrieNode {
    children: std::collections::HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode::default());
        }
        node.is_end_of_word = true;
    }

    pub fn search_prefixes(&self, prefix: &str, x: usize) -> Vec<String> {
        let mut node = &self.root;
        let mut result = Vec::new();
        let mut current_prefix = String::new();

        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => {
                    current_prefix.push(ch);
                    node = child;
                }
                None => return result, // Prefix not found, return empty result
            }
        }

        self.collect_prefixes(node, &mut current_prefix, x, &mut result);
        result
    }

    fn collect_prefixes(
        &self,
        node: &TrieNode,
        current_prefix: &mut String,
        x: usize,
        result: &mut Vec<String>,
    ) {
        if result.len() >= x {
            return;
        }

        if node.is_end_of_word {
            result.push(current_prefix.clone());
        }

        for (ch, child) in &node.children {
            current_prefix.push(*ch);
            self.collect_prefixes(child, current_prefix, x, result);
            current_prefix.pop();
        }
    }
}
