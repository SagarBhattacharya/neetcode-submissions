impl Solution {
  pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {

    if k == 0 { return vec![] };
    let k_idx = (k - 1) as usize;

    let mut maximums = vec![0i32; nums.len() - k_idx];
    let mut heap: BinaryHeap<(i32, usize)> =
      BinaryHeap::with_capacity(nums.len());

    for i in 0..nums.len() {
      heap.push((nums[i], i));
      
      if i >= k_idx {
        while let Some(&(_, idx)) =
          heap.peek() && idx < i - k_idx
        {
          heap.pop();
        }
        maximums[i-k_idx] = heap.peek()
          .copied().unwrap().0;
      }
    }

    maximums
  }
}