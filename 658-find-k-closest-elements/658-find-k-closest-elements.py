class Solution:
    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:
        left = 0
        right = len(arr) - k
        
        # 이분탐색으로
        while left < right:
            # 음수 제한
            mid = left + (right - left)//2
			
            if x - arr[mid] > arr[mid + k] - x:
                left = mid + 1
            else:
                right = mid
        
        return arr[left : left + k] # 범위 안 리턴
            