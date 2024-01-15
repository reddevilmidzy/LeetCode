# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if not root: return []
        
        res = [[]]
        queue = deque()
        queue.append((root, 0))
        
        while queue:
            cur, idx = queue.popleft()
            
            if not cur: continue
            
            if len(res) == idx:
                res.append([])
            res[idx].append(cur.val)
            queue.append((cur.left, idx+1))
            queue.append((cur.right, idx+1))

        return res