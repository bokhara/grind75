struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut char_curs: [i32; 58] = [0; 58];

        s.chars().for_each(|ch| {
            let index = ch as usize - 'A' as usize;
            char_curs[index] = char_curs[index] + 1;
        });
        let mut result = 0;
        let mut is_all_zero = true;
        for cur in char_curs {
            if cur > 0 {
                if cur % 2 == 0 {
                    result += cur;
                } else {
                    result += (cur - 1);
                    is_all_zero = false;
                }
            }
        }
        if is_all_zero {
            return result;
        }
        return result + 1;
    }
}
