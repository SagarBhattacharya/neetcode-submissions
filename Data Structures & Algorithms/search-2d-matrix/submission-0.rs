impl Solution {
  pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    use std::cmp::Ordering;
    let row_len = matrix.len() as i32;
    let col_len = matrix[0].len() as i32;

    let mut l = 0;
    let mut r = (row_len * col_len) - 1;
    while l <= r {
      let m = l + (r - l) / 2;
      match matrix
        [(m / col_len) as usize]
        [(m % col_len) as usize]
        .cmp(&target)
      {
        Ordering::Less => l = m + 1,
        Ordering::Equal => return true,
        Ordering::Greater => r = m - 1
      }
    }

    false
  }
}
