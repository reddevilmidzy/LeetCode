class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        s_len = len(s)
        t_len = len(t)
        t_idx = 0
        sub = ''
        
        for i in range(s_len):
            for j in range(t_idx, t_len):
                if s[i]==t[j]:
                    sub += s[i]
                    t_idx = j+1
                    break
        return sub==s