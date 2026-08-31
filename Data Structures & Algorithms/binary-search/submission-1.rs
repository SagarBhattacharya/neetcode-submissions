impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;

        let (mut l, mut r) = (0, nums.len().saturating_sub(1) as i32);
        while l <= r {
            let m = l + (r - l) / 2;
            match nums[m as usize].cmp(&target) {
                Ordering::Less => l = m + 1,
                Ordering::Greater => r = m - 1,
                Ordering::Equal => return m as i32,
            }
        }

        -1
    }
}
