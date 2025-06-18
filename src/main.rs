// Find All Numbers Diappeared in an Array 

use std::collections::HashSet;

fn main() {
    let nums = vec![4,3,2,7,8,2,3,1];
    let ans = Solution::find_disappeared_numbers(nums);
    println!("{ans:?}");
}


struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len() + 1;
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut ret = vec![];
        for i in 1..len {
            if !set.contains(&(i as i32)) {
                ret.push(i as i32);
            }
        }
        ret
    }
}
