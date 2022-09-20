class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        
        
        tmp = [[[] for _ in range(3)] for _ in range(3)]
        for i in range(9):
            row=[0]*(10)
            col=[0]*(10)
            for j in range(9):
                if board[i][j]!='.':
                    if row[int(board[i][j])]==0:
                        row[int(board[i][j])]=1
                    else:
                        return False
                
                    if board[i][j] in tmp[i//3][j//3]:
                        return False
                    else:
                        tmp[i//3][j//3].append(board[i][j])

                    
                if board[j][i]!='.':
                    if col[int(board[j][i])]==0:
                        col[int(board[j][i])]=1
                    else:
                        return False
        return True
                
                    