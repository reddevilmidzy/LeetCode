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
    private Map<Integer, List<Integer>> level = new HashMap<>();
    
    public boolean isEvenOddTree(TreeNode root) {
        dfs(root, 0);

        for (Map.Entry<Integer, List<Integer>> entry : level.entrySet()) {
            if (entry.getKey() % 2 == 0) {
                if (!isIncrease(entry.getValue()) || !isAllOdd(entry.getValue())) {
                    return false;
                }
            } else if (entry.getKey() % 2 == 1) {
                if (!isDecrease(entry.getValue()) || !isAllEven(entry.getValue())) {
                    return false;
                }
            }
        }
        return true;
    }

    private void dfs(TreeNode root, int depth) {
        if (root == null) {
            return;
        }

        dfs(root.left, depth + 1);
        List<Integer> tmp = level.getOrDefault(depth, new ArrayList<>());
        tmp.add(root.val);
        level.put(depth, tmp);
        dfs(root.right, depth + 1);
    }

    private boolean isIncrease(List<Integer> nodes) {
        for (int i = 0; i < nodes.size() - 1; i++) {
            if (nodes.get(i) >= nodes.get(i+1)) {
                return false;
            }
        }
        return true;
    }

    private boolean isDecrease(List<Integer> nodes) {
        for (int i = 0; i < nodes.size() - 1; i++) {
            if (nodes.get(i) <= nodes.get(i+1)) {
                return false;
            }
        }
        return true;
    }

    private boolean isAllOdd(List<Integer> nodes) {
        for (int node : nodes) {
            if (node % 2 == 0) {
                return false;
            }
        }
        return true;
    }

    private boolean isAllEven(List<Integer> nodes) {
        for (int node : nodes) {
            if (node % 2 != 0) {
                return false;
            }
        }
        return true;
    }
}