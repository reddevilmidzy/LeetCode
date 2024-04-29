class Solution:
    def minOperations(self, nums: List[int], k: int) -> int:
        tot = 0
        for num in nums:
            tot ^= num
        res = 0
        if tot == 0:
            return bin(k).count('1')
        
        n = math.ceil(math.log2(max(tot, k, 1))) + 1

        for i in range(n):
            if (tot >> i) & 1 != (k >> i) & 1:
                res += 1

        return res
