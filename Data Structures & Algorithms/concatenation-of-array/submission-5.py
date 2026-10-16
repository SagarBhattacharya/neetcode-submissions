class Solution:
    def getConcatenation(self, nums: List[int]) -> List[int]:
        copied = nums.copy()
        nums.extend(copied)
        return nums