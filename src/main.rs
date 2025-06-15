// Move Zeros

fn main() {
    let mut nums: Vec<i32> = vec![0,1,0,3,12];
    Solution::move_zeroes(&mut nums);
    println!("{nums:?}");

}


struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut not_zero = 0; 
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[not_zero] = nums[j];
                not_zero += 1;
            }
        }
        for el in nums.iter_mut().skip(not_zero) {
            *el = 0;
        }
    }
    
}
