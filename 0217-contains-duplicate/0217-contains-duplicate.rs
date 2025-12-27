impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable(); // Quicker than sort() because it doesn't preserve order of equal elements
        nums.windows(2).any(|pair| pair[0] == pair[1])
    }
}