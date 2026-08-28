class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        wi = 0
        for ri in range(len(nums)):
            if nums[ri] != val:
                nums[wi] = nums[ri]
                wi += 1
        return wi
