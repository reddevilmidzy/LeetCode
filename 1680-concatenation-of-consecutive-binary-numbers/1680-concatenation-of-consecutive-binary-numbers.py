class Solution:
    def concatenatedBinary(self, n: int) -> int:
        ans = [str(bin(i%(10**9+7)))[2:] for i in range(1, n+1)]
        # for i in range(1, n+1):
        #     binary += str(bin(i%(10**9+7)))[2:]
        return int(''.join(ans), 2)%(10**9+7)