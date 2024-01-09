# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        
        def bfs(root: Optional[TreeNode]):
            res = []
            stk = deque()
            stk.append(root)
            
            while stk:
                sub = stk.pop()
                if not sub:
                    continue
                elif sub.left == None and sub.right == None:
                    res.append(sub.val)
                else:
                    stk.append(sub.left)
                    stk.append(sub.right)
            
            return res

        return bfs(root1) == bfs(root2)