impl Solution {
    fn frequency(string: &String) -> [u8; 26] {
        let mut freq = [0u8; 26];
        for c in string.chars() {
            freq[c as usize - 'a' as usize] += 1;
        }
        freq
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        pub use std::collections::HashMap;
        let mut clusters: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for (i, string) in strs.iter().enumerate() {
            clusters.entry(Self::frequency(string))
                .and_modify(|lst| lst.push(string.clone()))
                .or_insert(vec![string.clone()]);
        }

        let mut result = vec![];
        for (_, strings) in clusters {
            result.push(strings);
        }

        result
    }
}
