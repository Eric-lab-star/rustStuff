

fn main() {
    let num_rows = 3;
    let a = Solution::get_row(num_rows);
    println!("{a:?}");
}


struct Solution;

impl Solution {
    pub fn get_row(row_index: i32)
    -> Vec<i32> {
        if row_index == 0 {
            return vec![1];
        }
        if row_index == 1 {
           return vec![1, 1];
        }
        let mut prev_row = vec![1, 1];
        for _ in 2..row_index + 1 {
            let mut tmp = vec![];
            for j in 0..prev_row.len() {
                if j + 1 > prev_row.len() - 1 {
                    break;
                }
                let a = prev_row[j];
                let b = prev_row[j + 1];
                tmp.push(a + b);
            }
            prev_row = tmp;
            prev_row.insert(0, 1);
            prev_row.push(1);
        }
        prev_row

    }

}

