class Solution:
    def freqKey(string: str):
        freq = [0] * 26
        for c in string:
            freq[ord(c) - ord("a")] += 1
        return tuple(freq)


    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        words = {}
        for s in strs:
            key = tuple(sorted(s))
            if key in words:
                words[key].append(s)
            else:
                words[key] = [s]

        ans = []
        for value in words.values():
            ans.append(value)
        return ans