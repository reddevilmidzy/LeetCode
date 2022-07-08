class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        ans = ""
        idx = 0
        try:
            for _ in range(len(strs[0])):
                for i in range(len(strs)-1):
                    if strs[i][idx] != strs[i+1][idx]:
                        break
                else:
                    ans += strs[0][idx]
                    idx += 1
        except:
            return ans
        return ans