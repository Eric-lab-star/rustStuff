use std::collections::HashMap;


fn main() {
    let nums = vec![99,99];
    let a = Solution::contains_nearby_duplicate(nums, 2);
    println!("{a}");
}


struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
       let mut last_index = HashMap::new();
       for (i, num) in nums.iter().enumerate() {
           if matches!(last_index.insert(num, i), Some(j) if i.abs_diff(j) <= k as usize ){
               return true;
           }
       }
       false
    }
}
