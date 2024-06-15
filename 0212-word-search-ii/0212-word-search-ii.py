class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        m, n = len(board), len(board[0])
        trie = Trie(words)
        seen = set()

        def adj(x, y):
            for i,j in [(0,-1),(0,1),(-1,0),(1,0)]:
                if 0 <= x+i < m and 0 <= y+j < n:
                    yield (x+i,y+j)
        
        def dfs(p, b, x, y):          
            ch, b[x][y] = b[x][y], "#"
            if trie.search(p):
                seen.add(p)
                trie.remove(p)

            for i,j in adj(x, y):
                if b[i][j] != "#":
                    pp = p + b[i][j]
                    if trie.starts(pp):
                        dfs(pp, b, i, j)
            
            b[x][y] = ch

        for i in range(m):
            for j in range(n):
                dfs(board[i][j], board, i, j)
        
        return seen
