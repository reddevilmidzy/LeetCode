class Solution:
    def can(self, position: List[int], target: int, m: int) -> int:
        pre = position[0]
        cnt = 1
        for num in position:
            if num - pre >= target:
                pre = num
                cnt += 1

        return cnt >= m


    def maxDistance(self, position: List[int], m: int) -> int:
        position.sort()

        left = 0
        right = int(1e9)
        res = 0
        while left <= right:
            mid = (left + right) // 2

            if self.can(position, mid, m):
                res = mid
                left = mid + 1
            else:
                right = mid - 1
        
        return res
