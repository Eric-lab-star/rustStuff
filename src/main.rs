
fn main() {
    let nums = vec![1,3,5,6];
    let a = Solution::search_insert(nums, 4);
    println!("a: {a}");

}
struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut k = 0;   
        for i in 0..nums.len() {
            match nums[i] {
                n if n >= target => {
                    return k;
                },
                n if n < target => {
                    k += 1;
                },
                _ => {}
            }
        }
        k
    }

}


