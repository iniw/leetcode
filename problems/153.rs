impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let min = nums[0];
        for n in nums {
            if n < min {
                return n;
            }
        }
        min
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
}
