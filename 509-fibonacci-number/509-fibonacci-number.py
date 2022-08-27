class Solution:
    def fib(self, n: int) -> int:
        dp=[0,1]
        if n <2:
            return dp[n]
        for i in range(2,n):
            a = dp.pop()
            b = dp.pop()
            dp.append(a)
            dp.append(a+b)
        return dp[0]+dp[1]