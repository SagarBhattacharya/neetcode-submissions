impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;

        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
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
