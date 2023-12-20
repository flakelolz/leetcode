#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut number = x;
        let mut result = 0;
        if x < 0 {
            return false;
        }
        while number != 0 {
            let pop = number % 10;
            number /= 10;
            result = result * 10 + pop;
        }
        x == result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_1() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn is_palindrome_2() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn is_palindrome_3() {
        assert!(!Solution::is_palindrome(10));
    }
}

