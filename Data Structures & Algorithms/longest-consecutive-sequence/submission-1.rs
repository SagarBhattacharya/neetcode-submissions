impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut num_set = HashSet::new();
        for num in nums {
            num_set.insert(num);
        }

        let mut lcs = 0;
        for num in num_set.iter() {
            if num_set.contains(&(num-1)) {
                continue;
            }

            let mut current = *num;
            let mut ccs = 1;
            while num_set.contains(&(current + 1)) {
                ccs += 1;
                current += 1;
            }

            lcs = lcs.max(ccs);
        }

        lcs
    }
}
