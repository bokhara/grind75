struct Solution {}
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut m_chars: [i32; 26] = [0; 26];
        for c in magazine.chars() {
            let idx = c as usize - 'a' as usize;
            m_chars[idx] = m_chars[idx] + 1;
        }
        for r in ransom_note.chars() {
            let rdx = r as usize - 'a' as usize;
            m_chars[rdx] = m_chars[rdx] - 1;
            if m_chars[rdx] < 0 {
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
    fn test_ransome_note1() {
        assert_eq!(
            Solution::can_construct(String::from("aa"), String::from("aab")),
            true
        );
    }
    #[test]
    fn test_ransome_note2() {
        assert_eq!(
            Solution::can_construct(String::from("a"), String::from("b")),
            false
        );
    }
    #[test]
    fn test_ransome_note3() {
        assert_eq!(
            Solution::can_construct(String::from("aa"), String::from("ab")),
            false
        );
    }
}
