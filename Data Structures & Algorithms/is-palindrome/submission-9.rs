impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let (mut start, mut end) = (0, bytes.len().saturating_sub(1));
        while start < end {
            if !bytes[start].is_ascii_alphanumeric() {
                start += 1;
                continue;
            }

            if !bytes[end].is_ascii_alphanumeric() {
                end -= 1;
                continue;
            }

            if bytes[start].to_ascii_lowercase() !=
                bytes[end].to_ascii_lowercase() 
            {
                return false;
            }

            start += 1;
            end -= 1;
        }
        true
    }
}
