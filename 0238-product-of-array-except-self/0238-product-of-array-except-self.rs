impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![1; n];
        
        // FIRST PASS: Fill with LEFT products
        let mut left_product = 1;
        for i in 0..n {
            answer[i] = left_product;
            left_product = left_product * nums[i];
        }
        
        // SECOND PASS: Multiply by RIGHT products
        let mut right_product = 1;
        for i in (0..n).rev() {
            answer[i] = answer[i] * right_product;
            right_product = right_product * nums[i];
        }
        
        answer
    }
}