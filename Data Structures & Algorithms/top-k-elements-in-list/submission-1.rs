impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let mut counts = HashMap::new();
        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut min_heap =
            BinaryHeap::with_capacity(k as usize + 1);

        for (num, count) in counts {
            min_heap.push(Reverse((count, num)));
            if min_heap.len() > k as usize {
            min_heap.pop();
            }
        }

        min_heap
            .into_iter()
            .map(|Reverse((_, num))| num)
            .collect()
    }
}
