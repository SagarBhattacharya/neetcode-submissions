from collections import defaultdict

class Solution:
    def frequency(self, s: str) -> Tuple[int, ...]:
        freq = [0] * 26
        for c in s:
            freq[ord(c) - ord('a')] += 1
        return tuple(freq)

    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        groups = defaultdict(list)
        for s in strs:
            groups[self.frequency(s)].append(s)
        return list(groups.values())
