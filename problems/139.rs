impl Solution {
    // https://leetcode.com/problems/word-break/solutions/6743981/video-using-dynamic-programming/
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 1..s.len() + 1 {
            for w in &word_dict {
                let start = i as i32 - w.len() as i32;
                if start >= 0 && dp[start as usize] && &s[start as usize..i] == w {
                    dp[i as usize] = true;
                    break;
                }
            }
        }

        return dp[s.len()];
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::word_break(
            "leetcode".to_owned(),
            vec!["leet".to_owned(), "code".to_owned()]
        ),
        true
    );
    assert_eq!(
        Solution::word_break(
            "applepenapple".to_owned(),
            vec!["apple".to_owned(), "pen".to_owned()]
        ),
        true
    );
    assert_eq!(
        Solution::word_break(
            "catsandog".to_owned(),
            vec![
                "cats".to_owned(),
                "dog".to_owned(),
                "sand".to_owned(),
                "and".to_owned(),
                "cat".to_owned()
            ]
        ),
        false
    );
    assert_eq!(
        Solution::word_break(
            "bb".to_owned(),
            vec![
                "a".to_owned(),
                "b".to_owned(),
                "bbb".to_owned(),
                "bbbb".to_owned()
            ]
        ),
        true
    );
    assert_eq!(
        Solution::word_break(
            "cars".to_owned(),
            vec!["car".to_owned(), "ca".to_owned(), "rs".to_owned(),]
        ),
        true
    );
}
