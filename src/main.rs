// Intersection of Two Arrays 

use std::collections::HashSet;

fn main() {
    let nums1 = vec![4,7,9,7,6,7]; 
    let nums2 = vec![5,0,0,6,1,6,2,2,4];
    let ans = Solution::intersection(nums1, nums2);
    println!("{ans:?}");

}


struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>)
    -> Vec<i32> {
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();
        let ans:Vec<i32> = set1.intersection(&set2).copied().collect();
        ans
    }

}
