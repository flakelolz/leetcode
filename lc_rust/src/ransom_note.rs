#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = std::collections::HashMap::new();
        for c in magazine.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        for c in ransom_note.chars() {
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
    fn can_construct_1() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();

        assert!(!Solution::can_construct(ransom_note, magazine));
    }

    #[test]
    fn can_construct_2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();

        assert!(!Solution::can_construct(ransom_note, magazine));
    }

    #[test]
    fn can_construct_3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();

        assert!(Solution::can_construct(ransom_note, magazine));
    }

    #[test]
    fn can_construct_4() {
        let ransom_note = "aab".to_string();
        let magazine = "baa".to_string();

        assert!(Solution::can_construct(ransom_note, magazine));
    }
}

