class Solution:
    def jump(self, nums: List[int]) -> int:
        n = len(nums)
        if n <= 2: 
            return n-1 # 무조건 갈 수 있음
        if n <= nums[0]: 
            return 1 # 1트에 성공하는 경우

        l, r = 0, nums[0]
        times = 1
        
        # 최근에 도착한 지점에서만 탐색
        while r < n - 1:
            times += 1
            nxt = max(i + nums[i] for i in range(l, r + 1))
            l, r = r, nxt
        return times