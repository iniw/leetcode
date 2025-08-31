#![allow(dead_code, unused_variables)]

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut count = 0;

        for c1 in 0..=limit {
            for c2 in 0..=limit {
                for c3 in 0..=limit {
                    if c1 + c2 + c3 == n {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::distribute_candies(5, 2), 3);
    assert_eq!(Solution::distribute_candies(3, 3), 10);
}
