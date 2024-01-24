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
   
    private int res = 0;
    private List<List<Integer>> allRoute = new ArrayList<>();
    public int pseudoPalindromicPaths (TreeNode root) {
        dfs(root, 0);
        return getRes();
    }
    
    private void dfs(TreeNode root, int route) {
        if (Objects.equals(root, null) || Objects.equals(root.val, null)) {
            return;
        }
        route ^= (1 << root.val);
        dfs(root.left, route);
        dfs(root.right, route);
        if (Objects.equals(root.left, null) && Objects.equals(root.right, null)) {
            if ((route & (route - 1)) == 0) {
                res++;
            }
        }
    }
    
    private int getRes() {
        return res;
    }
}