impl Solution {
  pub fn trap(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len().saturating_sub(1));
    let mut left_max = height[left];
    let mut right_max = height[right];

    let mut water = 0;
    while left < right {

      if left_max < right_max {
        left += 1;
        left_max = left_max.max(height[left]);
        water += left_max - height[left];
      } else {
        right -= 1;
        right_max = right_max.max(height[right]);
        water += right_max - height[right];
      }
    }
    water
  }
}
