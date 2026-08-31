impl Solution {
  pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut left = (0..n).collect::<Vec<_>>();
    let mut stack = vec![];

    for i in 0..n {
      while let Some(&idx) = stack.last() {
        if heights[idx] >= heights[i] {
          left[i] = left[idx];
          stack.pop();
        } else {
          break;
        }
      }
      stack.push(i);
    }

    stack.clear();
    let mut right = (0..n).collect::<Vec<_>>();
    for i in (0..n).rev() {
      while let Some(&idx) = stack.last() {
        if heights[idx] >= heights[i] {
          right[i] = right[idx];
          stack.pop();
        } else { break; }
      }
      stack.push(i);
    }

    heights.iter()
      .zip(right.iter())
      .zip(left.iter())
      .map(|((h, r), l)| h * (r - l + 1) as i32)
      .max()
      .unwrap()
  }
}