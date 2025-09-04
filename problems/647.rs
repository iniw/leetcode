impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut count = 0;
        for l in 0..s.len() {
            for r in l..s.len() {
                let substr = &s[l..=r];
                if substr.bytes().eq(substr.bytes().rev()) {
                    count += 1;
                }
            }
        }
        count
    }

    // https://leetcode.com/problems/palindromic-substrings/solutions/105689/java-solution-8-lines-extendpalindrome/
    pub fn count_substrings_v2(s: String) -> i32 {
        fn count(s: &[u8], mut l: i32, mut r: i32) -> i32 {
            let mut count = 0;
            while l >= 0 && r < s.len() as i32 && s[l as usize] == s[r as usize] {
                l -= 1;
                r += 1;

                count += 1;
            }
            count
        }

        let s = s.as_bytes();

        let mut total = 0;

        for i in 0..s.len() {
            let i = i as i32;
            total += count(s, i, i);
            total += count(s, i, i + 1);
        }

        total
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::count_substrings("abc".to_owned()), 3);
    assert_eq!(Solution::count_substrings("aaa".to_owned()), 6);
    assert_eq!(Solution::count_substrings_v2("abc".to_owned()), 3);
    assert_eq!(Solution::count_substrings_v2("aaa".to_owned()), 6);
}
