class Solution:
    def longestSubarray(self, nums: List[int], limit: int) -> int:
        n = len(nums)
        res = 1
        if n == 1:
            return res
        one = 0
        two = 1
        
        arr = {nums[one]: 1}
        if nums[two] in arr:
            arr[nums[two]] += 1
        else:
            arr[nums[two]] = 1
        max_val = max(arr.keys())
        min_val = min(arr.keys())

        while one <= two < n:
            if max_val - min_val <= limit:
                res = max(res, two - one + 1)
                two += 1
                if two == n: continue
                if nums[two] not in arr:
                    arr[nums[two]] = 0
                arr[nums[two]] += 1
                max_val = max(max_val, nums[two])
                min_val = min(min_val, nums[two])
            else:
                arr[nums[one]] -= 1
                if arr[nums[one]] == 0:
                    del arr[nums[one]]
                if nums[one] == min_val:
                    min_val = min(arr.keys())
                if nums[one] == max_val:
                    max_val = max(arr.keys())

                one += 1

        return res
