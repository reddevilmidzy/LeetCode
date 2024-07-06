class Solution:
    def passThePillow(self, n: int, time: int) -> int:
        x = ((n - 1) * 2)
        time %= x
        if time < n:
            return 1 + time
        return n - (time - n) - 1
