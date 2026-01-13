use std::collections::HashSet;



impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hashset = HashSet::with_capacity(nums.len()); // Optimization: pre-allocate memory
        
        for num in nums {
            // .insert() returns false if the element was already present
            if !hashset.insert(num) {
                return true;
            }
        }
        
        false
    }
}