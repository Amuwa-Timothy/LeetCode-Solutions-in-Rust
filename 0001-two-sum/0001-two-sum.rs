use std::collections::HashMap;


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut record = HashMap::new();

    for (index, value) in nums.iter().enumerate(){
        let complement = target - *value;

        if let Some(&prev_index) = record.get(&complement) {
            return vec![prev_index as i32, index as i32];
        }else{
        record.insert(*value, index);
        } 
    }    
    vec![]
    }
}
