# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> List[List[int]]:
        if not root:
            return []
        
        paths = []
        self.dfs(root,targetSum,[], paths)
        return paths

    def dfs(self, root, res, cur_path, paths):
        cur_path.append(root.val)
        if root.val == res and not (root.left or root.right):
            paths.append(cur_path)
            
        else:
            if root.left:
                self.dfs(root.left, res-root.val, cur_path[:], paths)
            if root.right:
                self.dfs(root.right, res-root.val, cur_path[:], paths)