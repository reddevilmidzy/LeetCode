class Solution:
    def getMaxLen(self, nums: List[int]) -> int:
        n = len(nums)
        dp = [[0] * 2 for _ in range(n)]   
        # 첫번째 인덱스는 양수 진행 두번째 인덱스는 음수 진행
        
        # 0이 오면 고르면 안됨
        if nums[0] > 0:
            dp[0][0] = 1
        
        if nums[0] < 0:
            dp[0][1] = 1
            
        res = dp[0][0] # 초기값
        
        for i in range(1, n):
            cur = nums[i]
            
            # 영이 아니라면
            if cur > 0:
                dp[i][0] = dp[i - 1][0] + 1
                if dp[i - 1][1] > 0: # if previous negative subarray exists
                    dp[i][1] = max(dp[i][1], dp[i - 1][1] + 1)
            if cur < 0:
                dp[i][1] = dp[i - 1][0] + 1
                if dp[i - 1][1] > 0: # if previous negative subarray exists
                    dp[i][0] = max(dp[i][0], dp[i - 1][1] + 1)
                    
            res = max(res, dp[i][0])
            
        return res