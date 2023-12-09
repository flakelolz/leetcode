struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = std::collections::HashMap::new();

        if s.len() != t.len() {
            return false;
        }

        for c in t.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        for c in s.chars() {
            if map.contains_key(&c) {
                if map[&c] <= 0 {
                    return false;
                }

                *map.get_mut(&c).unwrap() -= 1;
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_anagram_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();

        assert!(Solution::is_anagram(s, t));
    }
    
    #[test]
    fn is_anagram_2() {
        let s = "rat".to_string();
        let t = "car".to_string();

        assert!(!Solution::is_anagram(s, t));
    }
}

