

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    
    let mut freq = [0; 26];
    
    for ch in s.chars() {
        let index = (ch as u8 - b'a') as usize;
        freq[index] += 1;
    }
    
    for ch in t.chars() {
        let index = (ch as u8 - b'a') as usize;
        freq[index] -= 1;
    }
    
    freq.iter().all(|&count| count == 0)

    }
}