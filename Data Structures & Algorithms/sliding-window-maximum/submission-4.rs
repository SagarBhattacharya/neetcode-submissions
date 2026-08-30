impl Solution {
  pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 0 { return vec![] };
    let k_idx = (k - 1) as usize;

    let mut maximums = vec![0i32; nums.len() - k_idx];
    let mut queue = VecDeque::<usize>::new();

    for i in 0..nums.len() {
      while let Some(&idx) = queue.front() {
        if idx < i.saturating_sub(k_idx) {
          queue.pop_front();
        } else {
          break;
        }
      }
      
      while let Some(&idx) = queue.back() {
        if nums[idx] <= nums[i] {
          queue.pop_back();
        } else {
          break;
        }
      }
      
      queue.push_back(i);
      if i >= k_idx {
        maximums[i - k_idx] = nums[queue.front().copied().unwrap()];
      }
    }

    maximums
  }
}