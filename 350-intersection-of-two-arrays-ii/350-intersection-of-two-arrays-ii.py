class Solution:
    def intersect(self, nums1: List[int], nums2: List[int]) -> List[int]:
        cnt_nums1 = Counter(nums1)
        cnt_nums2 = Counter(nums2)
        
        ans = []
        for i in list(set(nums1)&set(nums2)):
            tmp = [i]*min(cnt_nums1[i], cnt_nums2[i])
            ans.extend(tmp)
        return ans
        