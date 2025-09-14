use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let pivot = nums.partition_point(|x| *x >= nums[0]) % nums.len();

        match (target.cmp(&nums[pivot]), target.cmp(&nums[nums.len() - 1])) {
            (Ordering::Equal, _) => pivot as i32,
            (_, Ordering::Equal) => nums.len() as i32 - 1,

            (Ordering::Greater, Ordering::Greater) => nums[..pivot]
                .binary_search(&target)
                .map(|i| i as i32)
                .unwrap_or(-1),

            (Ordering::Greater, Ordering::Less) => nums[pivot..]
                .binary_search(&target)
                .map(|i| (i + pivot) as i32)
                .unwrap_or(-1),

            (Ordering::Less, Ordering::Less) => -1,
            (Ordering::Less, Ordering::Greater) => -1,
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(Solution::search(vec![1], 0), -1);
    assert_eq!(Solution::search(vec![1, 3], 3), 1);
    assert_eq!(Solution::search(vec![3, 1], 1), 1);
    assert_eq!(Solution::search(vec![3, 1], 3), 0);
    assert_eq!(Solution::search(vec![5, 1, 3], 3), 2);
    assert_eq!(Solution::search(vec![3, 5, 1], 3), 0);
    assert_eq!(Solution::search(vec![5, 1, 3], 1), 1);
}
