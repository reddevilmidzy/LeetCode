class Solution:
    def maxSubarraySumCircular(self, nums: List[int]) -> int:
        tot, min_sum, min_cur, max_sum, max_cur = 0,nums[0],0,nums[0],0
        # 최대값을 구하는 방법은 배열에서 최대값을 찾거나, sum(배열)-최소값을 한 값이다.
        for i in nums:
            min_cur = min(min_cur+i, i)
            min_sum = min(min_sum, min_cur)
            max_cur = max(max_cur+i, i)
            max_sum = max(max_sum, max_cur)
            # sum 함수보다는 좀 더 빠른듯
            tot += i
        # 원소중에 음수도 있기에 한번 체크해줌    
        return max(max_sum, tot-min_sum) if max_sum > 0 else max_sum
            
        
        