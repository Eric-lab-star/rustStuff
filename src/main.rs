// teemo attacking 

fn main() {
   let timeSeries = vec![1,4];
   let duration = 2; 
   let ans = Solution::find_poisoned_duration(timeSeries, duration);
   println!("{ans}");
}

struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        time_series.iter().enumerate().skip(1).fold(duration, |acc, (index, &time)| {
            let interval = time - time_series[index - 1];
            if interval > duration {
                duration + acc
            } else {
                interval + acc
            }
        })

    }
}
