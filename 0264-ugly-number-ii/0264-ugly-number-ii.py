class Solution:
    def nthUglyNumber(self, n: int) -> int:
        if n <= 6:
            return n
        dp = [1]
        idx_2, idx_3, idx_5 = 0,0,0
        while n>1:
            time_2, time_3, time_5 = 2*dp[idx_2], 3*dp[idx_3], 5*dp[idx_5]
            minmum = min(time_2, time_3, time_5)
            if minmum == time_2:
                idx_2 += 1
                
            if minmum == time_3:
                idx_3 += 1
            
            if minmum == time_5:
                idx_5 += 1
            
            dp.append(minmum)
            n -= 1
        return dp[-1]
        