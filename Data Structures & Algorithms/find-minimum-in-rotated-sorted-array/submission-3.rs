impl Solution {
  pub fn find_min(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let (mut l, mut r) = (0, n as i32 - 1);
    
    while l < r {
      let m = l + (r - l) / 2;
      if nums[m as usize] > nums[r as usize] {
        l = m + 1;
      } else {
        r = m;
      }
    }

    nums[r as usize]
  }
}