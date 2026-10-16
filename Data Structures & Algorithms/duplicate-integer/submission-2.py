class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        normalized = set(nums)
        return len(normalized) != len(nums)