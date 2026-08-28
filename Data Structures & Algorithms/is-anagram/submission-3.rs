impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }
        let mut characters = [0usize; 26];
        for (s_char, t_char) in s.chars().zip(t.chars()) {
            characters[s_char as usize - 'a' as usize] += 1;
            characters[t_char as usize - 'a' as usize] -= 1;
        }
        characters.iter().all(|c| *c == 0)
    }
}
