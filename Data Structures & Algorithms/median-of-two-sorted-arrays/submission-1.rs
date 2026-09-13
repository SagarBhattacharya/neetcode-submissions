impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (shorter, longer) = if nums1.len() < nums2.len() 
        { (&nums1, &nums2) } else { (&nums2, &nums1) };

        let m = shorter.len() as i32;
        let n = longer.len() as i32;

        let k = (m + n + 1) / 2;
        let (mut l, mut r) = (0, m);

        while l <= r {
            let i = (l + r) / 2;
            let j = k - i;

            let left1 = if i == 0 { i32::MIN } else { shorter[i as usize - 1] };
            let right1 = if i == m { i32::MAX } else { shorter[i as usize] };

            let left2 = if j == 0 { i32::MIN } else { longer[j as usize -1] };
            let right2 = if j == n { i32::MAX } else { longer[j as usize] };

            if left1 > right2 {
                r = i - 1;
            } else if left2 > right1 {
                l = i + 1;
            } else {
                let left_max = left1.max(left2);
                let right_min = right1.min(right2);

                if (m + n) % 2 == 0 {
                    return (left_max + right_min) as f64 / 2.0;
                } else {
                    return left_max as f64;
                }
            }
        }
        
        0.0
    }
}
