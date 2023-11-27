#![allow(dead_code)]
use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let closed = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if closed.contains_key(&c) {
                let last = stack.pop();
                if last != Some(closed[&c]) {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_1() {
        let s = String::from("()");
        assert!(Solution::is_valid(s));
    }

    #[test]
    fn is_valid_2() {
        let s = String::from("()[]{}");
        assert!(Solution::is_valid(s));
    }

    #[test]
    fn is_valid_3() {
        let s = String::from("(]");
        assert!(!Solution::is_valid(s));
    }
}
