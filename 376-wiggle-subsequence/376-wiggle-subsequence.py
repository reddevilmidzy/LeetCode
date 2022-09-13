class Solution:
    def wiggleMaxLength(self, nums: List[int]) -> int:       
        length = 1
        up = None # 시작은 논값
        for i in range(1, len(nums)):
            if nums[i] > nums[i - 1] and up != True: # 위로 올라간거라면
                length += 1
                up = True
            if nums[i] < nums[i - 1] and up != False: # 이렇게 해준 이유는 첨에 논값임
                length += 1
                up = False
        return length