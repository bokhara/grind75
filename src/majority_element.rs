use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut num_count: HashMap<i32, i32> = HashMap::new();
        let len = nums.len();
        for num in nums {
            let count = num_count.get(&num).unwrap_or(&0);
            if (*count + 1) as usize > len / 2 {
                return num;
            }
            num_count.insert(num, *count + 1);
        }
        0
    }
    pub fn boyer_moore(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candinate = None;
        for num in nums {
            if count == 0 {
                candinate = Some(num);
            }
            if candinate.is_some() && num == candinate.unwrap() {
                count = count + 1;
            } else {
                count = count - 1;
            }
        }
        candinate.unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boyer_moore() {
        assert_eq!(Solution::boyer_moore(vec![2, 2, 3]), 2);
    }
}
