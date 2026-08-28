class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        length = 0
        min_len = min(len(s) for s in strs)
        for i in range(min_len):
            letter = strs[0][i]
            same = True
            for string in strs[1:]:
                if string[i] != letter:
                    same = False
            if same:
                length += 1
            else:
                break
        return strs[0][:length]