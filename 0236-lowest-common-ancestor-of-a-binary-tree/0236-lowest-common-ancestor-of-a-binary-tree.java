/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
class Solution {
    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        List<TreeNode> a = new ArrayList<>();
        List<TreeNode> b = new ArrayList<>();

        nodeToRootPath(root, p.val, a);
        nodeToRootPath(root, q.val, b);

        int i = a.size() - 1;
        int j = b.size() - 1;

        TreeNode prev = null;
        while (i >= 0 && j >= 0) {
            if (a.get(i).val != b.get(j).val) {
                break;
            }
            prev = a.get(i);
            i--;
            j--;
        }
        return prev;
    }

    private boolean nodeToRootPath(TreeNode root, int data, List<TreeNode> path) {
        if (root == null) {
            return false;
        }
        if (root.val == data) {
            path.add(root);
            return true;
        }
        boolean leftPath = nodeToRootPath(root.left, data, path);
        if (leftPath) {
            path.add(root);
            return true;
        }
        boolean rightPath = nodeToRootPath(root.right, data, path);
        if (rightPath) {
            path.add(root);
            return true;
        }
        return false;
    }
}