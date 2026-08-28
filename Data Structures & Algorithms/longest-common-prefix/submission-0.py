class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        length = 0
        min_len = min([len(s) for s in strs])
        for i in range(min_len):
            letter = set()
            for string in strs:
                letter.add(string[i])
            if len(letter) == 1:
                length += 1
            else:
                break
        return strs[0][:length]