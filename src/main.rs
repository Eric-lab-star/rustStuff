
fn main() {
    let mut nums1 = vec![4,5,6,0,0,0];
    let mut nums2 = vec![1,2,3];
    Solution::merge(&mut nums1,3, &mut nums2,3);
    println!("nums1: {nums1:?}");

}
// 3,8,9,5,0,0
// j     k
// 3,8,9,5,0,0
//   j   k
// 3,5,9,8,0,0
//   j   k
// 3,5,9,8,0,0
//     j k
//     8 9,0,0
//       j
// 

struct Solution;

impl Solution {
    pub fn merge(
        nums1: &mut Vec<i32>,
        m: i32,
        nums2: &mut Vec<i32>,
        n: i32
    ) {
        if m == 0 {
            for (i, num) in nums1.iter_mut().enumerate() {
                *num = nums2[i];
            }
            return;
        }


        for i in 0..n as usize {
            let num2 = nums2[i];
            let last = m as usize + i;
            nums1[last] = num2;
            for j in 0..last {
                if nums1[j] > nums1[last]{
                    let tmp = nums1[j];
                    nums1[j] = nums1[last];
                    nums1[last] = tmp;
                }

            }
        }
    }


}

// 9 -> 10
// 99 -> 100
// 299 -> 1000

// 29 -> 30

