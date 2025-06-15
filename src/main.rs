// Missing Number

fn main() {
    let nums = vec![9,6,4,2,3,5,7,0,1];
    let ans = Solution::missing_number(nums);
    println!("{ans:?}");
}


struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>)
    -> i32 {
        nums.into_iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc ^ (i + 1) as i32 ^ x)
    }
}


