class Solution:
    def canJump(self, nums: List[int]) -> bool:
        n = len(nums)
        goal = n-1 # 이곳만 방문하면 갈 수 있다는 의미  
        
        for i in range(n-1, -1, -1):
            if nums[i] + i >= goal: # tmp[i] + i >= n-1
                goal = i
        return goal == 0 # 0==0 => True 골이 0이라면 idx 0, 즉 0에 방문했음을 의미한다.
        
            