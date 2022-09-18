class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        # 좌우 최대값 담을 변수
        maxLeft, maxRight = [0] * n, [0] * n
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