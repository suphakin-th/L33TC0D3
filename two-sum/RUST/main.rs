use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (index1, n) in nums.into_iter().enumerate() {
            if let Some(&index2) = map.get(&n) {
                return vec![index1 as i32, index2 as i32];
            }