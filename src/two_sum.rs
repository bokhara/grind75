use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_pair = HashMap::new();
    for (index, val) in nums.iter().enumerate() {
        match num_pair.get(val) {
            Some(prev_index) => {
                return vec![*prev_index as i32, index as i32];
            }
            None => {
                num_pair.insert(target - val, index);
            }
        }
    }
    vec![-1, -1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
