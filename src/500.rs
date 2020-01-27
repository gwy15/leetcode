use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let rows : Vec<HashSet<char>> = vec![
            "qwertyuiop".to_string().chars().collect(),
            "asdfghjkl".to_string().chars().collect(),
            "zxcvbnm".to_string().chars().collect(),
        ];
        let mut mapper = HashMap::new();
        for row in rows.iter() {
            for ch in row.iter() {
                mapper.insert(ch, row);
            }
        }
        words.into_iter()
            .filter(|word| {
                let first_char = word.chars().next().unwrap();
                let first_char_lower_case = first_char.to_lowercase().collect::<Vec<char>>()[0];
                let row_set = &mapper[&first_char_lower_case];
                word.to_lowercase().chars().all(|ch| {
                    row_set.contains(&ch)
                })
            })
            .collect()
    }
}

