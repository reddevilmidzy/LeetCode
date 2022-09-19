class Solution:
    def matrixReshape(self, mat: List[List[int]], r: int, c: int) -> List[List[int]]:
        memo = []
        n = len(mat)
        m = len(mat[0])
        for i in range(n):
            for j in range(m):
                #print(mat[i][j])
                memo.append(mat[i][j])
        
        if n*m!=r*c:
            return mat
        
        #print(memo)
        ans = [[] for _ in range(r)]
        idx = 0
        for i in range(r):
            #ans.append([])
            for j in range(c):
                # print(memo[idx])
                ans[i].append(memo[idx])
                idx += 1
        return ans