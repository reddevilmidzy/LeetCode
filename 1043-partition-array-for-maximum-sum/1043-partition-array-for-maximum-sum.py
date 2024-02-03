class Solution:
    def maxSumAfterPartitioning(self, arr: List[int], k: int) -> int:
        # 문제에서 요구하는 것은
        # 최대값
        # 값이 클수록 k를 길게 가져가는 것이 유리
        n = len(arr)
        dp = [0]*(n+1)

        for i in range(1, n+1):
            mv = 0
            for j in range(1, min(k, i) + 1):
                mv = max(mv, arr[i-j])
                dp[i] = max(dp[i], dp[i-j] + mv*j)
                    
        return dp[n]