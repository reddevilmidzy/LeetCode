class Solution:
    def maxProfit(self, prices: List[int], fee: int) -> int:
        n = len(prices)
        # 길이가 2보다 작다면 안사고 안파는게 이득
        if n < 2:
             return 0
        ans = 0
        # 시작값
        minimum = prices[0]
        for i in range(1, n):
            if prices[i] < minimum:
                minimum = prices[i]
            elif prices[i] > minimum + fee: # 피 붙혀도 이득이라면
                ans += prices[i] - fee - minimum
                minimum = prices[i] - fee
        return ans