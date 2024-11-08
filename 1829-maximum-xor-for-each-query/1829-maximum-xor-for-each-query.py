class Solution:
    def getMaximumXor(self, nums: List[int], maximumBit: int) -> List[int]:
        # 전체 xor 하고, k xor 했을 때 최대 값. 
        # 이 k는 2^maximumbit 보다 작음
        # 가장 끝 숫자 제거
        n = len(nums)
        max_val = 2**maximumBit - 1
        tot = 0
        res = []
        for num in nums:
            tot ^= num
        for i in range(n-1, -1, -1):
            res.append(max_val ^ tot)
            tot ^= nums[i]
        return res