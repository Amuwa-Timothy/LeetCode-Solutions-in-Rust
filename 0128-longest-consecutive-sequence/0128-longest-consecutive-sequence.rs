use std::collections::HashSet;
use std::cmp::max;


impl Solution {

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

    
    let numbers: HashSet<i32> = nums.into_iter().collect();
    let mut longest_streak = 0;

    
    for num in &numbers {

        if !numbers.contains(&(*num - 1)) {
            let mut current_num = *num;
            let mut current_streak = 1;
            
            while numbers.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }
            

            longest_streak = max(longest_streak, current_streak);
        }
    }

    longest_streak
}






}