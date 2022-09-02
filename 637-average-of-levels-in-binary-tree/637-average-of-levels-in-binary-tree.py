# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def averageOfLevels(self, root: Optional[TreeNode]) -> List[float]:
        q, ans = deque([root]), [] # 넣고 뺄 자료구조
        while len(q): # 빌때까지
            qlen, row = len(q), 0
            for i in range(qlen):
                curr = q.popleft()
                row += curr.val
                if curr.left: q.append(curr.left)
                if curr.right: q.append(curr.right)
            ans.append(row/qlen) # 평균 넣기
        return ans
