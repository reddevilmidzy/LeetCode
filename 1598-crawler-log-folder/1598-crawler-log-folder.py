class Solution:
    def minOperations(self, logs: List[str]) -> int:
        cur = 0
        for log in logs:
            if log == "../":
                cur = max(0, cur - 1)
            elif log == "./":
                pass
            else:
                cur += 1
        
        return cur
