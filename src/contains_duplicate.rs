use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut num_map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            let cnt = num_map.get(&num).unwrap_or(&0);
            if *cnt == 1 {
                return true;
            }
            num_map.insert(num, *cnt + 1);
        }
        false
    }
}
