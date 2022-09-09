class Solution:
    def matrixBlockSum(self, mat: List[List[int]], k: int) -> List[List[int]]:
        m,n = len(mat), len(mat[0])
        sums = [[0]*(n+1) for _ in range(m+1)]
        
        
        for r in range(1, m+1):
            for c in range(1, n+1):
                sums[r][c] = mat[r-1][c-1] + sums[r-1][c] + sums[r][c-1] -sums[r-1][c-1]
                
        res = [[0]*n for _ in range(m)]
        
        for r in range(m):
            for c in range(n):
                r1 = max(0,r-k)+1
                c1 = max(0,c-k)+1
                r2 = min(m-1,r+k)+1
                c2 = min(n-1,c+k)+1
                
                res[r][c] = sums[r2][c2]-sums[r2][c1-1] - sums[r1-1][c2] + sums[r1-1][c1-1]
                
        return res
                
                