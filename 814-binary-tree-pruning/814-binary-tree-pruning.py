# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pruneTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        def dfs(node):
            if not node: # 이미 비었다면
                return True
            left, right = dfs(node.left), dfs(node.right) # 왼쪽 자식 탐색, 오른쪽 자식 탐색
            if left:
                node.left = None
            if right:
                node.right = None
            
            return left and right and node.val == 0 # lef == True and right == True and node.val == 0
        return root if not dfs(root) else None