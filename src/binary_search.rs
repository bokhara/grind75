pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return -1;
    }

    let mut beg = 0;
    let mut end = nums.len() - 1;
    while beg <= end {
        let mid = beg + (end - beg) / 2;

        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            if mid < nums.len() - 1 {
                beg = mid + 1;
            } else {
                return -1;
            }
        } else {
            if mid > 0 {
                end = mid - 1;
            } else {
                return -1;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(binary_search(vec![], 2), -1);
    }
    #[test]
    fn test_search1() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }
    #[test]
    fn test_search2() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn test_search3() {
        assert_eq!(binary_search(vec![-1, -1, -1, -1, -1], 2), -1);
    }

    #[test]
    fn test_search4() {
        assert_eq!(binary_search(vec![-1, -1, -1, -1, -1], -1), 2);
    }
    #[test]
    fn test_search5() {
        assert_eq!(binary_search(vec![-1], -1), 0);
    }

    #[test]
    fn test_search6() {
        assert_eq!(binary_search(vec![5], -5), -1);
    }

    #[test]
    fn test_search7() {
        assert_eq!(binary_search(vec![2, 5], 5), 1);
    }
}
