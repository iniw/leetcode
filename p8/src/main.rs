impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut s = s.trim().chars().peekable();

        let is_negative = matches!(s.next_if(|c| *c == '-' || *c == '+'), Some('-'));

        let mut result = 0i32;
        for c in s {
            if !c.is_numeric() {
                break;
            }

            result = result
                .saturating_mul(10)
                .saturating_add(c as i32 - '0' as i32);
        }

        if is_negative {
            -result
        } else {
            result
        }
    }
}

struct Solution;

fn main() {
    dbg!(Solution::my_atoi(
        "-001000000000000000000000000000".to_owned()
    ));
}
