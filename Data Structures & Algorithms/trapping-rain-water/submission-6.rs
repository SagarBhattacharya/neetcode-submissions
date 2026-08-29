impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut water = 0;

        for i in 0..height.len() {
            while !stack.is_empty() &&
                height[i] > height[stack.last().copied().unwrap()] 
            {
                let bottom = stack.pop().unwrap();
                if stack.is_empty() { break }

                let left_wall = stack.last().copied().unwrap();
                let region_width = i - 1 - left_wall;
                let region_height = height[i].min(height[left_wall]) - height[bottom];

                water += region_width as i32 * region_height;
            }
            stack.push(i);
        }
        water
    }
}
