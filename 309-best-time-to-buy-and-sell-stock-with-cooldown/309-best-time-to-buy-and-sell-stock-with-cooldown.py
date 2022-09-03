class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        # 잡고 있음
        notHold, notHold_cooldown, hold = 0, float('-inf'), float('-inf')
        for p in prices: # 하나씩 확인
            # (현재, 이전-현재),  
            hold, notHold, notHold_cooldown = max(hold, notHold - p), max(notHold, notHold_cooldown), hold + p
        return max(notHold, notHold_cooldown)