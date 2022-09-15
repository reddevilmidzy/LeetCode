class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        arr = sorted([i for i in coins if i<= amount])
        print(arr)
        if arr==[]:
            return 1
        elif amount==0:
            return 1
        
        dp = [0]*(amount+1)
        dp[0] = 1    
        for i in arr:
            for j in range(1,amount+1):
                if j>=i:
                    dp[j] += dp[j-i]
        
        print(dp)
        return dp[amount]
    
    