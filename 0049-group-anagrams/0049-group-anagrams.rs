use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<String, Vec<String>> = HashMap::new();

        for word in strs {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            let mut fingerprint: String = chars.into_iter().collect();
            anagrams.entry(fingerprint).or_insert(Vec::new()).push(word);
        }
        anagrams.into_values().collect()




        
    }
}

