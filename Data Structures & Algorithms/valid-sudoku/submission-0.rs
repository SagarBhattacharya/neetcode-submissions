impl Solution {
  fn is_valid_line(lines: &mut [u8; 9], line: &Vec<char>) -> bool {
    for num in line {
      if *num == '.' { continue; }
      lines[*num as usize - '1' as usize] += 1;
    }
    lines.iter().all(|v| *v <= 1)
  }

  pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let n = board.len();
    let mut lines = [0u8; 9];

    for i in 0..n {
      if !Self::is_valid_line(&mut lines, &board[i]) {
        return false;
      }
      lines.fill(0);
    }

    for i in 0..n {
      let column = board.iter().map(|row| row[i]).collect();
      if !Self::is_valid_line(&mut lines, &column) {
        return false;
      }
      lines.fill(0);
    }

    for i in 0..(n/3) {
      for j in 0..(n/3) {
        let sub_grid = board[i*3..(i+1)*3]
          .iter()
          .map(|row| row[j*3..(j+1)*3].to_vec())
          .flatten()
          .collect::<Vec<char>>();

        if !Self::is_valid_line(&mut lines, &sub_grid) {
          return false;
        }
        lines.fill(0);
      }
    }
    
    true
  }
}
