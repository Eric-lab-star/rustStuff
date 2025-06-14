// Summary Ranges

fn main() {
    let nums = vec![];
    let ans = Solution::summary_ranges(nums);
    println!("{ans:?}");
}


struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        let mut start: i32 = nums[0];
        let mut ans: Vec<String> = vec![];
        if nums.len() < 2 {
            ans.push(format!("{start}"));
            return ans;
        }

        for (i, &num) in nums.iter().enumerate().skip(1) {
            if num != nums[i - 1] + 1 {
                if start == nums[i - 1] {
                    ans.push(format!("{start}"));
                } else {
                    ans.push(format!("{start}->{}", nums[i - 1]));
                }
                start = nums[i];
            }
            if i == nums.len() - 1 {
                if start != nums[i] {
                    ans.push(format!("{start}->{}", nums[i]));
                } else {
                    ans.push(format!("{start}"));
                } 
            }
        }
        ans
    }
}
