# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pseudoPalindromicPaths (self, root: Optional[TreeNode]) -> int:
        
        def dfs(root, path):
            nonlocal ans
            if root == None:
                return
            # 시프트 연산
            path ^= (1 << root.val)
            if root.left == None and root.right == None:
                if path & (path - 1)==0:
                    ans += 1
                return
            
            # 좌 우
            dfs(root.left, path)
            dfs(root.right, path)
        
        
        ans = 0
        dfs(root,0)
        return ans