class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        res = set()
        n = len(nums)
        nums.sort()
        print(nums)
        for i in range(n):
            one = i + 1
            two = n - 1
            while one < two < n:
                val = nums[i] + nums[one] + nums[two]
                
                if val == 0:
                    res.add((nums[i], nums[one], nums[two]))
                    left = one + 1
                    while left < two < n and nums[one] == nums[left]:
                        # res.append((nums[i], nums[left], nums[two]))
                        # print('one', nums[i], nums[left], nums[two])
                        left += 1
                        
                    right = two - 1
                    while one < right < n and nums[two] == nums[right]:
                        # res.append((nums[i], nums[one], nums[right]))
                        # print('two', nums[i], nums[one], nums[right])
                        right -= 1
                    
                    one = left
                    two = right
                    
                elif val < 0:
                    one += 1
                else:
                    two -= 1
        return res