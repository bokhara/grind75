pub fn is_valid(s: String) -> bool {
    use std::collections::LinkedList;
    let mut list = LinkedList::new();
    for ch in s.chars() {
        let back = list.back();
        match back {
            Some(prev) => {
                if (*prev == '{' && ch == '}')
                    || (*prev == '[' && ch == ']')
                    || (*prev == '(' && ch == ')')
                {
                    list.pop_back();
                } else if *prev == '{' || *prev == '[' || *prev == '(' {
                    list.push_back(ch);
                } else {
                    return false;
                }
            }
            None => list.push_back(ch),
        }
    }
    list.is_empty()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(String::from("()")), true);
        assert_eq!(is_valid(String::from("()[]{}")), true);
        assert_eq!(is_valid(String::from("(}")), false);
        assert_eq!(is_valid(String::from("")), true);
        assert_eq!(is_valid(String::from(")(")), false);
        assert_eq!(is_valid(String::from(")")), false);
    }
}
