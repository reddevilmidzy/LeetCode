class Solution:
    def matrixReshape(self, mat: List[List[int]], r: int, c: int) -> List[List[int]]:
        memo = []
        n = len(mat)
        m = len(mat[0])
        if n*m!=r*c:
            return mat
        
        for i in range(n):
            for j in range(m):
                memo.append(mat[i][j])
        
        ans = [[] for _ in range(r)]
        idx = 0
        for i in range(r):
            for j in range(c):
                ans[i].append(memo[idx])
                idx += 1
        return ans