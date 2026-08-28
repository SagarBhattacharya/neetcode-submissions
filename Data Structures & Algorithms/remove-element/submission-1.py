class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        if len(nums) == 0:
            return 0
            
        scrap = max(nums)
        count = nums.count(val)

        for i in range(len(nums)):
            if nums[i] == val:
                nums[i] = scrap + 1
        nums.sort()

        return len(nums) - count