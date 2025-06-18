// Third Maximum Number

use std::collections::HashSet;

fn main() {
    let nums = vec![0,2];
    let a = Solution::third_max(nums);
    println!("{a}");
}

struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut sorted:Vec<i32> = set.into_iter().collect();
        sorted.sort_by(|a,b| b.cmp(a));
        println!("{sorted:?}");
        if sorted.len() >= 3 {
            return sorted[2]; }
        sorted[0]

    }
}
