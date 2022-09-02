class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        left = 0 #사는 포인트
        right = 1 # 파는 포인트
        max_profit = 0
        while right < len(prices):
            currentProfit = prices[right] - prices[left] # 현재
            if prices[left] < prices[right]:
                max_profit =max(currentProfit,max_profit)
            else: # 마이너스 난다면 다시 시작
                left = right
            right += 1
        return max_profit