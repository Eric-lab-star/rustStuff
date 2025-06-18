//  Assign Cookies

use std::collections::BinaryHeap;


fn main() {
    let g = vec![1,2];
    let s = vec![1,2,3];
    let ans = Solution::find_content_children(g, s);
    println!("{ans}");
}


struct Solution;

impl Solution {
   pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
       let mut ret = 0;
       let mut heap_g = BinaryHeap::from_iter(g);
       let mut heap_s = BinaryHeap::from_iter(s);
       while let Some(max_g) = heap_g.pop() {
           if let Some(&max_s) = heap_s.peek() {
               if max_s >= max_g {
                   ret += 1;
                   heap_s.pop();
               }
           }
       }
       ret
   } 
}
