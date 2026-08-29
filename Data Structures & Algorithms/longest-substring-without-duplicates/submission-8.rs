impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut window = [false; 128];

        let mut start = 0;
        let mut longest = 0;

        for (end, &c) in bytes.iter().enumerate() {
            while window[c as usize] {
                window[bytes[start] as usize] = false;
                start += 1;
            }
            window[c as usize] = true;
            longest = longest.max(end - start + 1);
        }

        longest as i32
    }
}
