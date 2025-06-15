// Range Sum Query - Immutable

fn main() {
    let nums = vec![-2, 0, 3, -5, 2, -1];
    let obj = NumArray::new(nums);
    let ret_1 = obj.sum_range(2, 5);
    println!("{ret_1}");

}

#[derive(Debug)]
struct NumArray {
    nums: Vec<i32>
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray {
            nums 
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.nums[left as usize..=right as usize].iter().sum()
    }
}


// struct Solution;
//
// impl Solution {
//     
// }
