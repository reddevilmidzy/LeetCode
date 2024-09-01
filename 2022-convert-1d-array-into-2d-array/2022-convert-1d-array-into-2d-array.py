class Solution:
    def construct2DArray(self, original: List[int], m: int, n: int) -> List[List[int]]:
        k = len(original)
        if n*m != k:
            return []
        res = [[0]*n for _ in range(m)]
        idx = 0
        for i in range(m):
            for j in range(n):
                res[i][j] = original[idx]
                idx+=1
        return res
