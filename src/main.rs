use std::collections::HashSet;



fn main() {
    let nums = vec![1,2,3,4];
    let a = Solution::contains_duplicate(nums);
    println!("{a}");
}


struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>)
    -> bool {
        let mut exists = HashSet::new();
        !nums.iter().all(|n| exists.insert(n))
    }
}
