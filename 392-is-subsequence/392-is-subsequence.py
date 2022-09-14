class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        s_len = len(s)
        t_len = len(t)
        s_idx = 0
        t_idx = 0
        sub = ''
        
        for i in range(s_len):
            for j in range(t_idx, t_len):
                if s[i]==t[j]:
                    sub += s[i]
                    t_idx = j+1
                    break
        #print(sub)
#         for i in range(t_len):
#             for j in range(s_idx, s_len):
#                 print('t',t[i], 's',s[j])
#                 if t[i]==s[j]:
#                     sub += t[i]
#                     s_idx = j+1
#                     break
#         print(sub)
        if sub==s:
            return True
        else:
            return False
#         # print('sub',sub)
