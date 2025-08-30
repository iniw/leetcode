impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        Self::generate_substrings(&s)
            .into_iter()
            .fold(0, |acc, s| acc + Self::count_unique_chars(s))
    }

    fn count_unique_chars(s: &str) -> i32 {
        let mut occurances = [0; 26];
        for c in s.bytes() {
            occurances[(c - b'A') as usize] += 1;
        }
        occurances
            .into_iter()
            .filter(|counter| *counter == 1)
            .count() as i32
    }

    fn generate_substrings(s: &str) -> Vec<&str> {
        let mut result = Vec::with_capacity(s.len() * s.len());
        for window in 1..=s.len() {
            for start in 0..(s.len() - window + 1) {
                result.push(&s[start..][..window])
            }
        }
        result
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::unique_letter_string("ABC".to_owned()), 10);
    assert_eq!(Solution::unique_letter_string("ABA".to_owned()), 8);
    assert_eq!(Solution::unique_letter_string("LEETCODE".to_owned()), 92);
}
