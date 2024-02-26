/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
class Solution {
    private boolean isEqual = true;
    
    public boolean isSameTree(TreeNode p, TreeNode q) {
        validateEquality(p, q);
        return isEqual;
    }
    
    private void validateEquality(TreeNode p, TreeNode q) {
        if (p == null && q == null) {
            return;
        } else if (p == null || q == null) {
            isEqual = false;
            return;
        }
        if (p.val != q.val) {
            isEqual = false;
            return;
        }
        validateEquality(p.left, q.left);
        validateEquality(p.right, q.right);
    }
}