class Solution:
    def rotateString(self, s: str, goal: str) -> bool:
        return goal in s + s[:-1] and len(goal) == len(s)
