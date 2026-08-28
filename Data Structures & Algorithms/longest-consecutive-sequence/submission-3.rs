impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let (mut l, mut c) = (1, 1);
        nums.sort_unstable();
        nums.dedup();

        for i in 0..nums.len() - 1 {
            if nums[i] + 1 == nums[i + 1] {
                c += 1;
                l = l.max(c);
            } else {
                c = 1;
            }
        }
        l
    }
}
