
fn main() {
    let num_rows = 5;
    let a = Solution::generate(num_rows);
    println!("{a:?}");
}


struct Solution;

impl Solution {
    pub fn generate(num_rows: i32)
    -> Vec<Vec<i32>> {
        if num_rows == 1 {
            return vec![vec![1]];
        }
        if num_rows == 2 {
           return vec![vec![1], vec![1, 1]]; 
        }
        let mut outer: Vec<Vec<i32>> = vec![vec![1], vec![1, 1]];
        for i in 2..num_rows {
            let prev_row = &outer[(i - 1) as usize];
            let mut inner: Vec<i32> = vec![];
            for j in 0..prev_row.len() {
                if j + 1 > prev_row.len() - 1 {
                    break;
                }
                let a = prev_row[j];
                let b = prev_row[j + 1];
                inner.push(a + b);
            }
            inner.insert(0, 1);
            inner.push(1);
            outer.push(inner);
        }
        outer
    }

}

