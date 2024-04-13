class Solution:
    def maximalRectangle(self, matrix: List[List[str]]) -> int:
        n = len(matrix)
        m = len(matrix[0])
        prefix = [[0]*(m+1) for _ in range(n+1)]
        for i in range(1, n+1):
            for j in range(1, m+1):
                prefix[i][j] = prefix[i][j-1] + prefix[i-1][j] + int(matrix[i-1][j-1]) - prefix[i-1][j-1]
        
        res = 0
        for y1 in range(1, n+1):
            for x1 in range(1, m+1):
                if matrix[y1-1][x1-1] == '0': continue
                if (m-x1+1)*(n-y1+1)<res: continue
                for y2 in range(y1, n+1):
                    for x2 in range(x1, m+1):
                        tmp = (x2-x1+1)*(y2-y1+1)
                        if prefix[y2][x2] - prefix[y1-1][x2] - prefix[y2][x1-1] + prefix[y1-1][x1-1] == tmp:
                            res = max(res, tmp)
                        else:
                            break
        return res
