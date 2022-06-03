use rand::distributions::uniform::SampleBorrow;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo: [i32; 45] = [0; 45];

        return Solution::climb_stairs_memo(n, &mut memo);
    }
    pub fn climb_stairs_memo(n: i32, memo: &mut [i32]) -> i32 {
        let memo_value = memo[(n - 1) as usize];
        if memo_value > 0 {
            return memo_value;
        }

        if n <= 2 {
            memo[(n - 1) as usize] = n;
            return n;
        } else {
            let result =
                Solution::climb_stairs_memo(n - 1, memo) + Solution::climb_stairs_memo(n - 2, memo);
            memo[(n - 1) as usize] = result;
            return result;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}
