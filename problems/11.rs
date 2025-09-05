impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;

        for l in 0..height.len() {
            for r in l..height.len() {
                let w = (r - l) as i32;
                let h = i32::min(height[r], height[l]);

                let area = w * h;
                if area > max {
                    max = area;
                }
            }
        }

        max
    }

    // https://leetcode.com/problems/container-with-most-water/solutions/5139915/video-simple-two-pointer-solution/
    pub fn max_area_v2(height: Vec<i32>) -> i32 {
        let mut max = 0;

        let mut l = 0;
        let mut r = height.len() - 1;

        while l != r {
            let w = (r - l) as i32;
            let h = i32::min(height[r], height[l]);

            let area = w * h;
            if area > max {
                max = area;
            }

            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        max
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
    assert_eq!(Solution::max_area_v2(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area_v2(vec![1, 1]), 1);
}
