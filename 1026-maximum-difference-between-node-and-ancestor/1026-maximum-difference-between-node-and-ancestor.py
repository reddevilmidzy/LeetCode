# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxAncestorDiff(self, root: Optional[TreeNode]) -> int:
        res = 0
        
        queue = deque()
        queue.append((root, root.val, root.val))
        
        while queue:
            rt, maxv, minv = queue.popleft()
            res = max(res, abs(rt.val - maxv), abs(rt.val - minv), maxv-minv)
            if not rt or type(rt) == int: continue
            

            if type(rt.left) == int:
                queue.append((rt.left, max(maxv, rt.left), min(minv, rt.left)))
            elif type(rt.left) == TreeNode:
                queue.append((rt.left, max(maxv, rt.left.val), min(minv, rt.left.val)))
            if type(rt.right) == int:
                queue.append((rt.left, max(maxv, rt.right), min(minv, rt.right)))
            elif type(rt.right) == TreeNode:
                queue.append((rt.right, max(maxv, rt.right.val), min(minv, rt.right.val)))
        
        return res