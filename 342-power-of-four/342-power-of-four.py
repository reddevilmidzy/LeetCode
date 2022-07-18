class Solution:
    def isPowerOfFour(self, n: int) -> bool:
        four = [4**i for i in range(17)]
        if n in four:
            return True
        else:
            return False
        