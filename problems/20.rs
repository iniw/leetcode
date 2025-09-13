impl Solution {
    pub fn is_valid(s: String) -> bool {
        enum Type {
            Paren,
            Curly,
            Brackets,
        }

        let mut stack = Vec::<Type>::with_capacity(s.len());

        for c in s.as_bytes() {
            match c {
                b'(' => stack.push(Type::Paren),
                b')' => match stack.pop() {
                    Some(Type::Paren) => continue,
                    _ => return false,
                },
                b'{' => stack.push(Type::Curly),
                b'}' => match stack.pop() {
                    Some(Type::Curly) => continue,
                    _ => return false,
                },
                b'[' => stack.push(Type::Brackets),
                b']' => match stack.pop() {
                    Some(Type::Brackets) => continue,
                    _ => return false,
                },
                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}

struct Solution;

fn main() {
    assert!(Solution::is_valid("()".to_owned()));
    assert!(Solution::is_valid("()[]{}".to_owned()));
    assert!(!Solution::is_valid("(]".to_owned()));
    assert!(Solution::is_valid("([])".to_owned()));
    assert!(!Solution::is_valid("([)]".to_owned()));
}
