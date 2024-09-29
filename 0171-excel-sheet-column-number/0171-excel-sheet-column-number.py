class Solution:
    def titleToNumber(self, columnTitle: str) -> int:
        n = len(columnTitle)
        return sum([(26 ** (n - i - 1)) * (ord(columnTitle[i]) - ord('A') + 1) for i in range(n)])
