use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        match (nums1.as_slice(), nums2.as_slice()) {
            ([], []) => 0.0,
            (nums, []) => Self::median(nums),
            ([], nums) => Self::median(nums),
            (nums1, nums2) => {
                let mut merged = Vec::with_capacity(nums1.len() + nums2.len());

                let mut nums1 = nums1.iter().copied().peekable();
                let mut nums2 = nums2.iter().copied().peekable();

                loop {
                    match (nums1.peek(), nums2.peek()) {
                        (Some(n1), Some(n2)) => match i32::cmp(n1, n2) {
                            Less => {
                                merged.push(*n1);
                                nums1.next();
                            }
                            Greater => {
                                merged.push(*n2);
                                nums2.next();
                            }
                            Equal => {
                                merged.push(*n1);
                                merged.push(*n2);
                                nums1.next();
                                nums2.next();
                            }
                        },
                        (Some(n), None) => {
                            merged.push(*n);
                            nums1.next();
                        }
                        (None, Some(n)) => {
                            merged.push(*n);
                            nums2.next();
                        }
                        (None, None) => break,
                    }
                }

                Self::median(&merged)
            }
        }
    }

    fn median(nums: &[i32]) -> f64 {
        let half_len = nums.len() / 2;
        if nums.len() % 2 == 0 {
            let m1 = nums[half_len];
            let m2 = nums[half_len - 1];
            (m1 + m2) as f64 / 2.0
        } else {
            nums[half_len] as f64
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_median_sorted_arrays(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 4],
            vec![1, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4],
        ),
        3.0
    );
}
