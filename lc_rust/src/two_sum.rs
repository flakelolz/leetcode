#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash: HashMap<i32, i32> = HashMap::new();

        for (i, &n) in nums.iter().enumerate() {
            let diff = target - n;
            if hash.contains_key(&diff) {
                return vec![hash[&diff], i as i32];
            }
            hash.insert(n, i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(Solution::two_sum_hashmap(nums.clone(), target), vec![0, 1]);
        assert_eq!(Solution::two_sum_force(nums, target), vec![0, 1]);
    }

    #[test]
    fn two_sum_2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        assert_eq!(Solution::two_sum_hashmap(nums.clone(), target), vec![1, 2]);
        assert_eq!(Solution::two_sum_force(nums, target), vec![1, 2]);
    }

    #[test]
    fn two_sum_3() {
        let nums = vec![3, 3];
        let target = 6;

        assert_eq!(Solution::two_sum_hashmap(nums.clone(), target), vec![0, 1]);
        assert_eq!(Solution::two_sum_force(nums, target), vec![0, 1]);
    }
}
