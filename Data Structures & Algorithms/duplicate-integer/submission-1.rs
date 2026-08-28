impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        set.extend(nums.iter().cloned());
        set.len() != nums.len()
    }
}
