impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let want = n * (n + 1) / 2;
        let got = nums.into_iter().sum::<i32>();
        want - got
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    assert_eq!(Solution::missing_number(vec![1]), 0);
}
