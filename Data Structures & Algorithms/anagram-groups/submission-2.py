class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        s = {i: sorted(s) for i, s in enumerate(strs)}
        s = dict(sorted(s.items(), key=lambda item: item[1]))

        ans = []
        current = []
        current_idx = []
        for key, value in s.items():
            if len(current) == 0 or value in current:
                current.append(value)
                current_idx.append(key)
            else:
                ans.append(current_idx)
                current = [value]
                current_idx = [key]

        if len(current) != 0:
            ans.append(current_idx)

        for i in range(len(ans)):
            ans[i] = [strs[j] for j in ans[i]]

        return ans