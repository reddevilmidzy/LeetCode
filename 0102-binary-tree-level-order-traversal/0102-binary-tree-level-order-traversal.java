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
    public List<List<Integer>> levelOrder(TreeNode root) {
        List<List<Integer>> res = new ArrayList<>();
        if (root == null) {
            return res;
        }
        res.add(new ArrayList<>());
        Queue<Node> queue = new LinkedList<>();
        queue.add(new Node(0, root));
        
        while (!queue.isEmpty()) {
            Node cur = queue.poll();
            TreeNode tree = cur.root;
            if (Objects.equals(tree, null) || Objects.equals(tree.val, null)) {
                continue;
            }
            if (res.size() == cur.idx) {
                res.add(new ArrayList<>());
            }
            List<Integer> tmp = res.get(cur.idx);
            tmp.add(tree.val);
            queue.add(new Node(cur.idx+1, tree.left));
            queue.add(new Node(cur.idx+1, tree.right));
        }
        return res;
    }
    
    static class Node {
        private int idx;
        private TreeNode root;
        
        public Node(int idx, TreeNode root) {
            this.idx = idx;
            this.root = root;
        }
    }
}