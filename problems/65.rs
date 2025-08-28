use std::{iter::Peekable, str::Chars};

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut s = s.trim().chars().peekable();

        Self::consume_sign(&mut s);

        let found = match s.next() {
            Some('.') => match s.peek() {
                Some('0'..='9') => Found::Dot,
                _ => return false,
            },
            Some('0'..='9') => {
                Self::consume_numbers(&mut s);
                match s.next() {
                    Some('.') => Found::Dot,
                    Some('e' | 'E') => Found::Exponent,
                    Some(_) => return false,
                    None => return true,
                }
            }
            _ => return false,
        };

        match found {
            Found::Dot => match s.next() {
                Some('e' | 'E') => Self::consume_after_exponent(&mut s),
                Some('0'..='9') => {
                    Self::consume_numbers(&mut s);
                    match s.next() {
                        Some('e' | 'E') => Self::consume_after_exponent(&mut s),
                        Some(_) => false,
                        None => true,
                    }
                }
                Some(_) => false,
                None => true,
            },
            Found::Exponent => Self::consume_after_exponent(&mut s),
        }
    }

    fn consume_numbers(s: &mut Peekable<Chars>) -> bool {
        let mut found = false;
        while s.next_if(|c| c.is_numeric()).is_some() {
            found = true;
        }
        found
    }

    fn consume_after_exponent(s: &mut Peekable<Chars>) -> bool {
        Self::consume_sign(s);
        Self::consume_numbers(s) && s.next().is_none()
    }

    fn consume_sign(s: &mut Peekable<Chars>) {
        s.next_if(|c| *c == '-' || *c == '+');
    }
}

enum Found {
    Dot,
    Exponent,
}

struct Solution;

fn main() {
    assert_eq!(Solution::is_number("123".to_owned()), true);
}
