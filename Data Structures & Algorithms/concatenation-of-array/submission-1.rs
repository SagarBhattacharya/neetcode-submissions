impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; 2 * n];
        for (i, num) in nums.into_iter().enumerate() {
            ans[i] = num;
            ans[i + n] = num;
        }
        return ans
    }
}
