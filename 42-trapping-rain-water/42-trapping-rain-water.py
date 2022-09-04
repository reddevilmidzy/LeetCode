class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        # 정방향으로 갈때 가장 높은 거 담을 배열과, 역방향으로 갈 때 가장 높은 거 담을 배열
        maxLeft, maxRight = [0] * n, [0] * n
        # 집어넣음
        for i in range(1, n):
            maxLeft[i] = max(height[i-1], maxLeft[i-1])
        for i in range(n-2, -1, -1):
            maxRight[i] = max(height[i+1], maxRight[i+1])
            
        ans = 0
        for i in range(n):
            # 수심은 더 작은거 체크
            waterLevel = min(maxLeft[i], maxRight[i])
            if waterLevel >= height[i]:  # 수심이 높다면
                ans += waterLevel - height[i]
        return ans