use std::mem;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_product = i32::MIN;
        for l in 0..nums.len() {
            for r in l + 1..=nums.len() {
                let subarray = &nums[l..r];
                let product = subarray.iter().product();
                max_product = max_product.max(product);
            }
        }
        max_product
    }

    // https://leetcode.com/problems/maximum-product-subarray/solutions/48230/possibly-simplest-solution-with-o-n-time-complexity/
    pub fn max_product_v2(nums: Vec<i32>) -> i32 {
        let mut r = nums[0];

        let mut max = r;
        let mut min = r;

        for n in nums.into_iter().skip(1) {
            if n < 0 {
                mem::swap(&mut max, &mut min);
            }

            max = i32::max(n, max * n);
            min = i32::min(n, min * n);

            r = i32::max(r, max);
        }

        r
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);

    assert_eq!(Solution::max_product_v2(vec![2, 3, -2, 4]), 6);
    assert_eq!(Solution::max_product_v2(vec![-2, 0, -1]), 0);
}
