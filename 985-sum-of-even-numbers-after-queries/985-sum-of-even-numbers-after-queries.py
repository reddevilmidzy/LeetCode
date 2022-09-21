class Solution:
    def sumEvenAfterQueries(self, nums: List[int], queries: List[List[int]]) -> List[int]:
        ans = []
        tmp = sum([i for i in nums if i%2==0]) # 짝수를 미리 더해둠
        
        for val,idx in queries:
            pre = nums[idx]
            nums[idx] += val
            
            if pre%2==0:
                tmp-=pre
            if nums[idx]%2==0:
                tmp += nums[idx]
            ans.append(tmp)
            
        return ans