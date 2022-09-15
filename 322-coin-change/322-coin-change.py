class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        INF = int(1e9)
        dp = [INF]*(amount+1)
        arr = sorted([i for i in coins if i <= amount])
        
        if amount==0:
            return 0
        elif arr==[]:
            return -1
        print(arr)
        
        for coin in arr:
            dp[coin] = 1
            
        for i in range(arr[0]+1, amount+1):
            for coin in arr:
                if i > coin and dp[i-coin] != INF:
                    # print(i, coin)
                    dp[i] = min(dp[i-coin]+1,dp[i])
        # print(dp[480:])
        
        return dp[amount] if dp[amount] != INF else -1