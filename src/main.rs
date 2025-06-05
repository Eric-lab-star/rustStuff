
fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let a = Solution::remove_element(&mut nums, 3);
    println!("nums: {:?}, a: {a}", nums);

}
struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }

}


