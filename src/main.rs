use std::collections::HashMap;


fn main() {
    let nums = vec![1,2,3,4];
    let a = Solution::contains_duplicate(nums);
    println!("{a}");
}


struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>)
    -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in nums.iter() {
            if map.contains_key(num) {
                return true;
            } else {
                map.insert(*num, 1);
            }
        }
        false

    }
}
