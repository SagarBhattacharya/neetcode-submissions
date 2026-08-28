impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;

        let (mut start, mut end) = (0, numbers.len().saturating_sub(1));
        while start < end {
            let expected = numbers[start] + numbers[end];
            match target.cmp(&expected) {
                Ordering::Less => end -= 1,
                Ordering::Equal => return vec![start as i32 + 1, end as i32 + 1],
                Ordering::Greater => start += 1,
            }
        }

        vec![]
    }
}
