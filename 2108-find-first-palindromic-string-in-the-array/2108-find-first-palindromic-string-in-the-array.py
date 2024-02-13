class Solution:
    def isPalindrome(self, word: str) -> bool:
        n = len(word)
        if n%2==0:
            return word[:n//2] == word[n//2:][::-1]
        return word[:n//2+1] == word[n//2:][::-1]
    
    def firstPalindrome(self, words: List[str]) -> str:
        for word in words:
            if self.isPalindrome(word):
                return word
        return ""
    