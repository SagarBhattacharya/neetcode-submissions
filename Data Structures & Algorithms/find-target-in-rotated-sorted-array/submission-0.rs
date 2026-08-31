impl Solution {
  pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let (mut l, mut r) = (0, n as i32 - 1);

    while l <= r {
      let m = l + (r - l) / 2;
      if target == nums[m as usize] {
        return m;
      }

      if nums[l as usize] <= nums[m as usize] {
        if (nums[l as usize]..=nums[m as usize]).contains(&target) {
          r = m - 1;
        } else {
          l = m + 1;
        }
      } else if nums[m as usize] <= nums[r as usize] {
        if (nums[m as usize]..=nums[r as usize]).contains(&target) {
          l = m + 1;
        } else {
          r = m - 1;
        }
      }
    }

    -1
  }
}
