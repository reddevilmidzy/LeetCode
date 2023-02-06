class Solution:
    def shuffle(self, nums: List[int], n: int) -> List[int]:
        first = nums[:n]
        second = nums[n:]
        res = []
        for i in range(n):
            res.append(first[i])
            res.append(second[i])
        
        return res