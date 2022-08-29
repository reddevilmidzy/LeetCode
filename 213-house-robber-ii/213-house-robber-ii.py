class Solution:
    def rob(self, nums: List[int]) -> int:
        n = len(nums)
        if n < 4:
            return max(nums)
        elif n == 4:
            return max(nums[0]+nums[2], nums[1]+nums[3])
        elif n == 5:
            return max(nums[0]+nums[2], nums[1]+nums[3], nums[2]+nums[4], nums[0]+nums[3], nums[1]+nums[4])
        else:
            on_dp = [0]*n
            in_dp = [0]*n
            re_nums = nums[::-1]
            for i in range(3):
                on_dp[i] = nums[i]
                in_dp[i] = re_nums[i]
            
            for i in range(2, n-1):
                on_dp[i] = max(on_dp[i-2]+nums[i], on_dp[i-1], on_dp[i-3]+nums[i])
                in_dp[i] = max(in_dp[i-2]+re_nums[i], in_dp[i-1], in_dp[i-3]+re_nums[i])
            
            return max(on_dp[n-1],on_dp[n-2],in_dp[n-1],in_dp[n-2])