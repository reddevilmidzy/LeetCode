# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    cur = 0
    def bstToGst(self, root: TreeNode) -> TreeNode:
        self.search(root)
        return root

    def search(self, root: TreeNode) -> None:       
        if root == None:
            return
        self.search(root.right)
        self.cur += root.val
        root.val = self.cur
        self.search(root.left)
