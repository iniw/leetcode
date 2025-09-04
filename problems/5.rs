impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest = "";

        for l in 0..s.len() {
            for r in (l + longest.len())..s.len() {
                let substr = &s[l..=r];
                if substr.bytes().eq(substr.bytes().rev()) {
                    longest = substr;
                }
            }
        }

        longest.to_owned()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::longest_palindrome("babad".to_owned()),
        "bab".to_owned()
    );
    assert_eq!(
        Solution::longest_palindrome("cbbd".to_owned()),
        "bb".to_owned()
    );
    assert_eq!(Solution::longest_palindrome("b".to_owned()), "b".to_owned());
}
