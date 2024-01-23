class Solution:    
    def maxLength(self, arr: List[str]) -> int:
        candy = []
        ans = 0
        for s in arr:
            if len(s) == len(set(s)):
                candy.append(s)

        def bi(s: str) -> int:
            cur_bit = 0
            for c in s:
                cur_bit |= 1 << (ord(c) - ord('a'))
            
            return cur_bit

        def bt(idx: int, arr: List[str], cur: int):
            nonlocal ans
            ans = max(ans, bin(cur).count('1'))

            if idx == len(arr):
                return

            
            for nxt in range(idx+1, len(arr)):
                
                new_bit = bi(arr[nxt])
                
                if cur&new_bit == 0: # 겹치는게 없음
                    bt(nxt, arr, cur|new_bit)
                else:
                    bt(nxt, arr, cur)
        
        bt(-1, candy, 0)
        
        return ans
