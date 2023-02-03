class Solution:
    def convert(self, s: str, numRows: int) -> str:
        
        if numRows == 1:
            return s
        n = len(s)
        ans = ["" for _ in range(numRows)]
        
        idx = 0
        pat = 0
        down = True
        cnt = 0
        while idx < n: 
            ans[pat] += s[idx]
            if cnt == numRows-1:
                cnt = 0
                down = not down

            if down:
                pat += 1
                cnt += 1
            else:
                pat -= 1
                cnt += 1
            
            idx += 1
        
        return "".join(ans)