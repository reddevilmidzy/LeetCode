class Solution:
    def integerBreak(self, n: int) -> int:
        if n<4:
            return n-1
        product = 1
        while n>4:
            product*=3
            n -= 3

        product*=n

        return product
