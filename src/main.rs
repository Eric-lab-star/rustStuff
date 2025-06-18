//  Assign Cookies



fn main() {
    let g = vec![1,2];
    let s = vec![1,2,3];
    let ans = Solution::find_content_children(g, s);
    println!("{ans}");
}


struct Solution;

impl Solution {
   pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
       g.sort();
       s.sort();

       let mut gi = 0;
       let mut si = 0;
       loop {
           if si >= s.len() || gi >= g.len() {
               break;
           }
           if g[gi] <= s[si] {
               gi += 1;
           }
           si += 1;
       }
       gi as i32
   } 
}
