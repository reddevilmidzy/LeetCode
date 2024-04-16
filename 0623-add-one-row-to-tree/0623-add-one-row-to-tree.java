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
    public TreeNode addOneRow(final TreeNode root, final int val, final int depth) {
        if (depth == 1) {
            final TreeNode child = new TreeNode(root.val, root.left, root.right);
            root.val = val;
            root.left = child;
            root.right = null;
            return root;
        }
        search(root, val, depth, 1);
        return root;
    }

    private void search(final TreeNode root, final int val, final int depth, final int current) {
        if (root == null) {
            return;
        }
        if (current + 1 == depth) {
            final TreeNode left = root.left;
            final TreeNode right = root.right;
            final TreeNode newRowLeft = new TreeNode(val, left, null);
            final TreeNode newRowRight = new TreeNode(val, null, right);
            root.left = newRowLeft;
            root.right = newRowRight;
            return;
        }

        search(root.left, val, depth, current + 1);
        search(root.right, val, depth, current + 1);
    }
}
