struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_byte = a.into_bytes();
        let mut b_byte = b.into_bytes();
        a_byte.reverse();
        b_byte.reverse();
        let mut result = "".to_string();
        let len = if a_byte.len() < b_byte.len() {
            a_byte.len()
        } else {
            b_byte.len()
        };
        let mut augement = 0;
        let mut idx = 0;
        while idx < len {
            let mut val = (a_byte[idx] as usize - '0' as usize)
                + (b_byte[idx] as usize - '0' as usize)
                + augement;
            augement = val / 2;
            val = val % 2;
            result = val.to_string() + &result;
            idx = idx + 1;
        }
        while idx < a_byte.len() {
            let mut val = (a_byte[idx] as usize - '0' as usize) + augement;
            augement = val / 2;
            val = val % 2;
            result = val.to_string() + &result;
            idx = idx + 1;
        }
        while idx < b_byte.len() {
            let mut val = (b_byte[idx] as usize - '0' as usize) + augement;
            augement = val / 2;
            val = val % 2;
            result = val.to_string() + &result;
            idx = idx + 1;
        }
        if augement > 0 {
            result = augement.to_string() + &result;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_binary() {
        assert_eq!(
            Solution::add_binary(String::from("11"), String::from("1")),
            String::from("100")
        )
    }
    #[test]
    fn test_add_binary2() {
        assert_eq!(
            Solution::add_binary(String::from("1010"), String::from("1011")),
            String::from("10101")
        )
    }
    #[test]
    fn test_add_binary3() {
        assert_eq!(
            Solution::add_binary(String::from("1111"), String::from("1111")),
            String::from("11110")
        )
    }
}
