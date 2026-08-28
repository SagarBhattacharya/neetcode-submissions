class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        available = []
        for i, val in enumerate(nums):
            if val in available:
                return [available.index(val), i]
            available.append(target - val)
        return None