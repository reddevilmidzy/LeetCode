class Solution:
    def resultGrid(self, image: List[List[int]], threshold: int) -> List[List[int]]:
        n = len(image)
        m = len(image[0])
        
        pre = [[0]*(m+1) for _ in range(n+1)]
        for i in range(1, n+1):
            for j in range(1, m+1):
                pre[i][j] = pre[i-1][j] + pre[i][j-1] + image[i-1][j-1] - pre[i-1][j-1]
        
        dy = (1,0,-1,0)
        dx = (0,1,0,-1)
        
        def can_(y: int, x: int) -> bool:
            
            for i in range(4):
                ny,nx = dy[i]+y, dx[i]+x
                if abs(image[y][x]-image[ny][nx]) > threshold:
                    # print("1")
                    return False
            
            if abs(image[y-1][x-1]-image[y-1][x]) > threshold:
                # print("2")
                return False
            if abs(image[y+1][x-1]-image[y+1][x]) > threshold:
                # print("3")
                return False
            if abs(image[y-1][x-1]-image[y][x-1]) > threshold:
                # print("4")
                return False
            if abs(image[y+1][x-1]-image[y][x-1]) > threshold:
                # print("5")
                return False
            
            if abs(image[y-1][x+1]-image[y-1][x]) > threshold:
                # print("6")
                return False
            if abs(image[y-1][x+1]-image[y][x+1]) > threshold:
                # print("7")
                return False
            if abs(image[y+1][x+1]-image[y+1][x]) > threshold:
                # print("8")
                return False
            if abs(image[y+1][x+1]-image[y][x+1]) > threshold:
                # print("9")
                return False
            
            return True
        
        dy = (1,0,-1,0, 1,1,-1,-1, 0)
        dx = (0,1,0,-1, 1,-1,1,-1, 0)
        board = defaultdict(list)
#         print('image')
#         print(*image, sep='\n')
#         print('pre')
#         print(*pre, sep='\n')
        
        for y in range(1, n-1):
            for x in range(1, m-1):
                # print("y",y, "x", x)
                if can_(y,x):
                    # print("y", y, "x", x)
                    ay,ax = y+1,x+1
                    by,bx = y-1, x-1
                    val = pre[ay+1][ax+1] - pre[ay+1][bx] - pre[by][ax+1] + pre[by][bx]
                    # print("y",y, "x", x, "val", val)
                    val //= 9

                    # print(pre[ay+1][ax+1] , pre[by-2][bx] , pre[by][bx-2] , pre[by][bx])
                    for i in range(9):
                        ny,nx = dy[i]+y, dx[i]+x            
                        board[(ny,nx)].append(val)
        
        res = image[:]
        for y,x in board.keys():
            res[y][x] = sum(board[(y,x)])//len(board[(y,x)])
        return res