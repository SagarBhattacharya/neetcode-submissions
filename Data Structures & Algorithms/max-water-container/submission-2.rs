impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let (mut start, mut end) = (0, heights.len().saturating_sub(1));

        while start < end {
            let min_height = heights[start].min(heights[end]);
            max_area = max_area.max(min_height * (end - start) as i32);

            if heights[start] == min_height {
                start += 1;
                continue;
            }

            if heights[end] == min_height {
                end -= 1;
                continue;
            }
        }

        max_area
    }
}
