impl Solution {
  pub fn two_sum(nums: &[i32], target: i32, original_idx: usize) -> Vec<(i32, i32)> {
    use std::collections::HashSet;
    let mut sums = HashSet::new();
    let mut sets = vec![];

    for (i, num) in nums.iter().enumerate() {
      if i == original_idx { continue; }
      let remaining = target - *num;
      if sums.contains(num) {
        sets.push((*num, remaining));
      }
      sums.insert(remaining);
    }
    sets
  }

  pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    let mut results = HashSet::new();
    
    for (i, num) in nums.iter().enumerate() {
      for (n2, n3) in Self::two_sum(&nums, -num, i) {
        let mut result = vec![*num, n2, n3];
        result.sort();
        results.insert(result);
      }
    }
    results.into_iter().collect()
  }
}