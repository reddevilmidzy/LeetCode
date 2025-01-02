class Solution:
    def vowelStrings(self, words: List[str], queries: List[List[int]]) -> List[int]:
        vowels = {"a", "e", "i", "o", "u"}
        pre = [0]
        for word in words: pre.append(pre[-1] + (word[0] in vowels and word[-1] in vowels))
        return [pre[r+1] - pre[l] for l,r in queries]