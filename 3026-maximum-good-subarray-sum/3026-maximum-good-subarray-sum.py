class Solution:
    def maximumSubarraySum(self, nums: List[int], k: int) -> int:
        n = len(nums)
        mp = dict()
        res = -float('inf')
        pre = [0]
        for num in nums:
            pre.append(pre[-1] + num)
        
        # print(pre)
        for i in range(1, n+1):
            if nums[i-1] not in mp:
                mp[nums[i-1]] = float('inf')
            mp[nums[i-1]] = min(mp[nums[i-1]], pre[i-1])
            
            if nums[i-1]-k in mp:
                res = max(res, pre[i] - mp[nums[i-1]-k])
            if nums[i-1]+k in mp:
                # print('여기', nums[i-1]+k, nums[i-1])
                res = max(res, pre[i] - mp[nums[i-1]+k])
        # print(mp)
        if res == -float('inf'):
            return 0
        return res