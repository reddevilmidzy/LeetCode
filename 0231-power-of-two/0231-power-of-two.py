class Solution:
    def isPowerOfTwo(self, n: int) -> bool:
        num = bin(n)[2:]
        return num.count("1") == 1 and num[0] == "1"