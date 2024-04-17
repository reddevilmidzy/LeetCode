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

    private final List<String> words = new ArrayList<>();

    public String smallestFromLeaf(final TreeNode root) {
        search(root, "");
        Collections.sort(words);
        return words.get(0);
    }

    public void search(final TreeNode root, final String pre) {
        if (root == null) {
            return;
        }
        final String cur = String.valueOf((char) (root.val + 'a'));
        search(root.left, cur + pre);
        search(root.right, cur + pre);
        if (root.left == null && root.right == null) {
            words.add(cur + pre);
        }
    }
}
