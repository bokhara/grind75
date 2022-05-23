// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, 
// typically using all the original letters exactly once.
// s and t consist of lowercase English letters.
pub fn is_anagram(s:String, t: String) -> bool {
    let mut s1:[i32;26] = [0;26];
    for ch in s.chars() {
        let idx = ch as usize - 'a' as usize;
        s1[idx] = s1[idx] + 1;
    }
    let mut t1:[i32;26] = [0;26];
    for ch in t.chars() {
        let idx = ch as usize - 'a' as usize;
        t1[idx] = t1[idx] + 1;
    }
    
    s1 == t1

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_angram() {
        assert_eq!(is_anagram(String::from("cat"), String::from("act")),true);
    }

    #[test]
    fn test_is_angram2() {
        assert_eq!(is_anagram(String::from("cat"), String::from("fct")),false);
    }
}