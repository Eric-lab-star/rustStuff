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
        let expect: i32 = (((nums.len() as i32) + 1) * nums.len() as i32)/2;
        let mut sum: i32 = 0;
        for num in nums {
            sum += num;
        }
        expect - sum
    }
}


