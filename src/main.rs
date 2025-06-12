use std::collections::HashMap;

fn main() {
    let nums = vec![2,2,4,1,1];
    let a = Solution::single_number(nums);
    println!("{a}");
}


struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>)
    -> i32 {
       let mut numbers_map: HashMap<i32, i32> = HashMap::new(); 
       for num in nums.iter() {
           if !numbers_map.contains_key(num){
               numbers_map.insert(*num, 1);
           } else {
               numbers_map.remove(num);
           }
       }
       nums.iter().fold(0, |acc, cur| acc ^ cur)
    }
}
