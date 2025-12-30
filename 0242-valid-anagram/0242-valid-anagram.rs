use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        
        let mut map = HashMap::new();
        
        for letter in s.chars() {
            *map.entry(letter).or_insert(0) += 1;
        }
        

        for letter in t.chars() {
            let count = map.entry(letter).or_insert(0);
            *count -= 1;
        }

        map.values().all(|&count| count == 0)
    }
}