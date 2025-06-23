// Next Greater Element 1


fn main() {
    let nums1 = vec![2,4];
    let nums2 = vec![1,2,3,4];
    let ans = Solution::next_greater_element(nums1, nums2);
    assert_eq!(ans, [3,-1]);
}

struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut answer:Vec<i32> = vec![-1; nums1.len()];

        for i in 0..nums2.len() {
            while stack.last().map_or(false, |&l| l < nums2[i]) {
                let val = stack.pop().unwrap();
                if let Some(j) = nums1.iter().position(|&n| n == val) {
                    answer[j] = nums2[i];
                }
            }
            stack.push(nums2[i]);
        }
        answer
    }
}
