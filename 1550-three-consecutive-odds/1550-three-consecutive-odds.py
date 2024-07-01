class Solution:
    def threeConsecutiveOdds(self, arr: List[int]) -> bool:
        n = len(arr)
        if n < 3: return False
        for i in range(n-2):
            if arr[i]%2 == arr[i+1]%2 == arr[i+2]%2 == 1:
                return True
        return False
