
fn main() {
    let nums = vec![2,2,3];
    let a = Solution::majority_element(nums);
    println!("{a}");
}


struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>)
    -> i32 {
        let mut score = 0;
        let mut candidate = nums[1];

        for num in nums {
            if score == 0 {
                candidate = num;
            }

            if candidate == num {
                score += 1;
            } else {
                score -= 1;
            }
        }

        candidate


    }
}
