use std::collections::HashMap;


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut record = HashMap::new();

    for (index, value) in nums.iter().enumerate(){
      if record.contains_key(&(target - *value )){
        return vec![*record.get(&(target - *value)).unwrap() as i32, index as i32];
      }else{
        record.insert(*value, index);
       }
    }    
    vec![]
    }
}
