class Solution:
    def removeOccurrences(self, s: str, part: str) -> str:
        stk = []
        n = len(part)
        p = list(part)
        for c in s:
            stk.append(c)
            while len(stk) >= n and stk[-n:] == p:
                for _ in range(n):
                    stk.pop()
                
        return ''.join(stk)