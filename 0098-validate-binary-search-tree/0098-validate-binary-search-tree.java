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

    private List<Integer> route = new ArrayList<>();

    public boolean isValidBST(TreeNode root) {
        search(root);
        int n = route.size();

        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                if (route.get(i) >= route.get(j)) {
                    return false;
                }
            }
        }

        return true;
    }

    private void search(TreeNode root) {
        if (root == null) {
            return;
        }

        search(root.left);
        route.add(root.val);
        search(root.right);
    }
    
}