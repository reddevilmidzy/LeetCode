class Solution:
    def reverseParentheses(self, s: str) -> str:
        stk = []
        res = []

        for i in s:
            if i == "(":
                stk.append(len(res))
            elif i == ")":
                st = stk.pop()
                res[st:] = res[st:][::-1]
            else:
                res.append(i)
        
        return ''.join(res)
        