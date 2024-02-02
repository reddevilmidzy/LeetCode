class Solution:
    def sequentialDigits(self, low: int, high: int) -> List[int]:
        s = "".join([str(i) for i in range(1, 10)])
        res = []
        n = len(str(low))
        m = len(str(high))
        
        for p in range(n, m+1):
            for q in range(10-p):
                if low <= int(s[q:q+p]) <= high:
                    print(s[q:q+p])
                    res.append(int(s[q:q+p]))
        return res