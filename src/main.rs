use std::collections::HashMap;

fn main() {
    let nums = vec![2,2,3];
    let a = Solution::majority_element(nums);
    println!("{a}");
}


struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>)
    -> i32 {
        if nums.len() < 2 {
            return nums[0]
        }
        let mut map: HashMap<i32, i32> = HashMap::new(); 
       for num in nums.iter() {
           if let Some(v) = map.get_mut(num) {
               *v += 1;
               if *v > ((nums.len() as i32) / 2) {
                   return *num
               }
           } else {
               map.insert(*num, 1);
           }
       }
       0
    }
}
