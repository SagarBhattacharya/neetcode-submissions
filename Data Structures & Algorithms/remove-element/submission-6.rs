impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut wi = 0i32;
        for ri in 0..nums.len() {
            if nums[ri] != val {
                nums[wi as usize] = nums[ri];
                wi += 1;
            }
        }
        wi
    }
}
