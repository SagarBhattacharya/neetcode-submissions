impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;

        for i in 0..=heights.len() {
            let current = if i == heights.len() {0} 
              else {heights[i]};

            while let Some(&top) = stack.last() {
                if heights[top] > current {
                    stack.pop();
                    let height = heights[top];
                    let left = stack.last()
                        .map_or(0, |&idx| idx + 1);
                    let width = i - left;
                    max_area = max_area.max(height * width as i32);
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        max_area
    }
}