class Solution:
    def intersect(self, nums1: List[int], nums2: List[int]) -> List[int]:
        cnt1 = Counter(nums1)
        cnt2 = Counter(nums2)
        tmp = []
        for i in cnt1&cnt2:
            tmp.extend([i]*min(cnt1[i], cnt2[i]))
        return tmp
