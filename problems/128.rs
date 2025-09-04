use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.iter().copied().collect::<HashSet<i32>>();

        let mut max_seq = 0;

        for n in nums {
            if !set.contains(&(n - 1)) {
                let mut seq = 1;
                while set.contains(&(n + seq)) {
                    seq += 1;
                }
                max_seq = i32::max(max_seq, seq);
            }
        }

        max_seq
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(
        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
        9
    );
    assert_eq!(Solution::longest_consecutive(vec![1, 0, 1, 2]), 3);
}
