use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map = nums
            .iter()
            .enumerate()
            .map(|(i, n)| (*n, i))
            .collect::<HashMap<i32, usize>>();

        for (i, n) in nums.into_iter().enumerate() {
            let delta = target - n;
            if let Some(j) = map.get(&delta)
                && *j != i
            {
                return vec![i as i32, *j as i32];
            }
        }

        unreachable!()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
