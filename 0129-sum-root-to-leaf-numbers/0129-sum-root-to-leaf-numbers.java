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
    private final List<Integer> values = new ArrayList<>();

    public int sumNumbers(final TreeNode root) {
        search(root, 0);
        return values.stream()
                .mapToInt(Integer::valueOf)
                .sum();
    }

    private void search(final TreeNode root, final int value) {
        if (root == null) {
            return;
        }
        if (root.left == null && root.right == null) {
            values.add(value * 10 + root.val);
        }
        search(root.left, value * 10 + root.val);
        search(root.right, value * 10 + root.val);
    }
}
