class Solution:
    def merge(self, nums1: List[int], nums2: List[int]) -> List[int]:
        n1, n2 = len(nums1), len(nums2)
        result = [0] * (n1 + n2)

        i, j, k = 0, 0, 0
        while i < n1 and j < n2:
            if nums1[i] < nums2[j]:
                result[k] = nums1[i]
                i += 1
                k += 1
            else:
                result[k] = nums2[j]
                j += 1
                k += 1
        
        while i < n1:
            result[k] = nums1[i]
            i += 1
            k += 1
        
        while j < n2:
            result[k] = nums2[j]
            j += 1
            k += 1

        return result

    def sortArray(self, nums: List[int]) -> List[int]:
        if len(nums) <= 1:
            return nums
        
        mid = len(nums) // 2
        left = self.sortArray(nums[:mid])
        right = self.sortArray(nums[mid:])
        return self.merge(left, right)