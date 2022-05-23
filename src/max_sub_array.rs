pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len as i32 == 1 {
        return nums[0];
    }
    let mut max = nums[0];
    let mut current_max = 0;

    for n in nums {
        current_max = std::cmp::max(n, n + current_max);
        if current_max > max {
            max = current_max;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subarray() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn test_max_subarray2() {
        assert_eq!(max_sub_array(vec![1]), 1);
    }

    #[test]
    fn test_max_subarray3() {
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
