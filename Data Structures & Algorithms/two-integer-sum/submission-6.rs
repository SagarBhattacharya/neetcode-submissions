impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut candidates = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(idx) = candidates.get(num) {
                return vec![*idx as i32, i as i32]
            }
            candidates.insert(target-num, i);
        }
        vec![]
    }
}
