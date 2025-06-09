use std::{cell::RefCell, rc::Rc};

fn main() {
    let nums = vec![-10,-3,0,5,9];

    let a = Solution::sorted_array_to_bst(nums);
    println!("{a:?}");

}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

}

struct Solution;

impl Solution {
    pub fn solve(nums: &Vec<i32>, start: i32, end: i32)
    -> Option<Rc<RefCell<TreeNode>>>{
        if start > end {
            return None;
        }
        let mid = (end + start) / 2;

        let root = Rc::new(RefCell::new(TreeNode::new(nums[mid as usize])));
        root.borrow_mut().left = Solution::solve(nums, start, mid - 1);
        root.borrow_mut().right = Solution::solve(nums, mid + 1, end);
        Some(root)


    }
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::solve(&nums, 0, (nums.len() - 1) as i32)

    }
    
}

