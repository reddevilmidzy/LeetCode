class Solution:
    def maxSumAfterPartitioning(self, arr: List[int], k: int) -> int:
        # 문제에서 요구하는 것은
        # 최대값
        # 값이 클수록 k를 길게 가져가는 것이 유리
        n = len(arr)
        dp = [0]*(n+1)

        for i in range(n+1):
            for j in range(1, k+1):
                if arr[i-j:i]:
                    dp[i] = max(dp[i], dp[i-j] + max(arr[i-j:i])*j)
                    # print(dp, max(arr[i-j:i]*j), "i", i, "j", j)
        
        # print(dp)
        return dp[n]