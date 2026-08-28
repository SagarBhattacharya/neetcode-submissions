impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = Vec::new();

        if nums.len() < 3 {
            return result;
        }

        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                break;
            }

            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let target = -nums[i];
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[left] + nums[right];
                if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                } else {
                    result.push(vec![
                        nums[i],
                        nums[left],
                        nums[right],
                    ]);

                    let left_value = nums[left];
                    let right_value = nums[right];

                    while left < right && nums[left] == left_value {
                        left += 1;
                    }

                    while left < right && nums[right] == right_value {
                        right -= 1;
                    }
                }
            }
        }

        result
    }
}