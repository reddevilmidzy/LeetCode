class Solution:
    def numTrees(self, n: int) -> int:
        return factorial(2*n)//(factorial(n+1)**2//(n+1))
        # [(1,1), (2, 2), (3,5), (4, 14), (5, 42), (6, 132), (7,429)]