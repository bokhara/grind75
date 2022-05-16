pub fn is_palindrome(s: String) -> bool {
    let helper = |c: char| match c.is_alphanumeric() {
        true => Some(c.to_ascii_lowercase()),
        false => None,
    };
    s.chars()
        .filter_map(helper)
        .eq(s.chars().rev().filter_map(helper))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(String::from("")), true)
    }
    #[test]
    fn test_is_palindrome1() {
        assert_eq!(
            is_palindrome(String::from("A man, a plan, a canal: Panama")),
            true
        )
    }
    #[test]
    fn test_is_palindrome2() {
        assert_eq!(is_palindrome(String::from("race a car")), false)
    }

    #[test]
    fn test_is_palindrome3() {
        assert_eq!(is_palindrome(String::from(" ")), true)
    }
}
