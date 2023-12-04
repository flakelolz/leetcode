#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if let Some(&value) = nums.get(mid) {
                match value.cmp(&target) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Greater => right = mid - 1,
                    std::cmp::Ordering::Equal => return mid as i32,
                }
            };
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;

        assert_eq!(Solution::search(nums, target), 4);
    }

    #[test]
    fn search_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;

        assert_eq!(Solution::search(nums, target), -1);
    }
}

