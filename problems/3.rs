use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();

        let mut best = 0;

        for i in 0..s.len() {
            let mut seen = HashSet::new();
            for nc in &s[i..] {
                if !seen.insert(nc) {
                    break;
                }
            }
            best = i32::max(best, seen.len() as i32);
        }

        best
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_owned()),
        3
    );
    assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
    assert_eq!(
        Solution::length_of_longest_substring("pwwkew".to_owned()),
        3
    );
    assert_eq!(Solution::length_of_longest_substring(" ".to_owned()), 1);
}
