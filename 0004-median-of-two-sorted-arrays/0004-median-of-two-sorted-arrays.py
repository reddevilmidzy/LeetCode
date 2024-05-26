class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        nums1.extend(nums2)
        nums1.sort()
        m = len(nums1)//2
        if len(nums1)%2:
            return nums1[m]
        return (nums1[m-1] + nums1[m]) / 2
