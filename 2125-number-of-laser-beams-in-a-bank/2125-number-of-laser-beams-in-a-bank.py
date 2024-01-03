class Solution:
    def numberOfBeams(self, bank: List[str]) -> int:
        n = len(bank)
        m = len(bank[0])
        pre = 0
        res = 0
        for i in range(n):
            tmp = 0
            for j in range(m):
                if bank[i][j] == "1":
                    tmp += 1
            if tmp:
                res += tmp*pre
                pre = tmp
            
        return res