
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;


impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq_map = HashMap::new();
        for num in nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }
        
        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        
        for (num, freq) in freq_map {
            min_heap.push(Reverse((freq, num)));
            

            if min_heap.len() > k as usize {
                min_heap.pop();
            }
        }
        
        min_heap
            .into_iter()
            .map(|Reverse((_, num))| num)
            .collect()
    }
}