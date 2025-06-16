// Intersection of Two Arrays 

use std::collections::HashSet;

fn main() {
    let nums1 = vec![4,9,5]; 
    let nums2 = vec![9,4,9,8,4];
    let ans = Solution::intersection(nums1, nums2);
    println!("{ans:?}");

}


struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>)
    -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut set: HashSet<i32> = HashSet::new();
        for num in nums1 {
            set.insert(num);
        }
        for num2 in nums2 {
            if !set.insert(num2) && !res.contains(&num2) {
                res.push(num2);
            }
        }
        res
    }

}
