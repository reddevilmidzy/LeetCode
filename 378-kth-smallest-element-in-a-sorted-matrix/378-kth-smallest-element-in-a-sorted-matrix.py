class Solution:
    def kthSmallest(self, matrix: List[List[int]], k: int) -> int:
        ans=[]
        for i in matrix:
            ans.extend(i)
        ans=sorted(ans)
        return ans[k-1]