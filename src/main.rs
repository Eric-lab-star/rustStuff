
fn main() {
    let nums = vec![9,9,9];
    let a = Solution::plus_one(nums);
    println!("a: {a:?}");

}
struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut p = digits.len() - 1;
        let mut num = digits[p];
        let mut tens = 0;
        num += 1;
        if num >= 10 {
            num = 0;
            tens = 1;
        }

        let mut new_digits = vec![0; digits.len()];
        new_digits[p] = num;
        p -= 1;

        for _ in 1..digits.len() {
            let mut num = digits[p];
            if tens != 0 {
                num += tens;
                tens = 0;
            }
            if num >= 10 {
                tens += 1;
                num %= 10;
            }
            new_digits[p] = num;
            if p != 0{
                p = p.saturating_sub(1);
            }

        }
        if tens != 0 {
            new_digits.insert(0, 1);
        }

        new_digits
    }

}

// 9 -> 10
// 99 -> 100
// 299 -> 1000

// 29 -> 30

