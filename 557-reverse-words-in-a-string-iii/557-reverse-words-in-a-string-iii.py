class Solution:
    def reverseWords(self, s: str) -> str:
        ans = []
        word = list(map(str,s.split()))
        for i in word:
            ans.append(i[::-1])
        print(word)
        return ' '.join(ans)