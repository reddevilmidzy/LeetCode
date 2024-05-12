class Solution(object):
    def canPartition(self, nums):
        def subsetSumEqualToK(index, nums, target, dp):
            for i in range(index):            
                dp[i][0] = True

            for idx in range(1, len(nums)):
                for tgt in range(1, target+1):
                    not_take = dp[idx - 1][tgt]
                    take = False
                    if nums[idx] <= tgt:
                        take = dp[idx-1][tgt-nums[idx]]
                    dp[idx][tgt] = take or not_take
    
            return dp[len(dp)-1][len(dp[0])-1]

        total_sum = 0

        for num in nums:
            total_sum += num

        if total_sum&1==1:
            return False

        desired_sum = total_sum//2

        target = desired_sum
        dp=[[False for i in range(target + 1)] for j in range(len(nums))]
        return subsetSumEqualToK(len(nums), nums, desired_sum, dp)
