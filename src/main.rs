// Third Maximum Number

use std::collections::{BinaryHeap, HashSet};

fn main() {
    let nums = vec![3,2,1];
    let a = Solution::third_max(nums);
    println!("{a}");
}

struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut heap = BinaryHeap::from_iter(set);
        if heap.len() >= 3 {
            heap.pop();
            heap.pop();
            heap.pop().unwrap_or(0)
        } else {
            heap.pop().unwrap_or(0)
        }
    }
}
