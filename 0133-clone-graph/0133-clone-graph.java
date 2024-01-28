/*
// Definition for a Node.
class Node {
    public int val;
    public List<Node> neighbors;
    public Node() {
        val = 0;
        neighbors = new ArrayList<Node>();
    }
    public Node(int _val) {
        val = _val;
        neighbors = new ArrayList<Node>();
    }
    public Node(int _val, ArrayList<Node> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
}
*/

class Solution {
    public Node cloneGraph(Node node) {
        if (node == null) {
            return null;
        }
        
        Queue<Node> queue = new LinkedList<>();
        queue.add(node);
        Set<Integer> visited = new HashSet<>();
        visited.add(node.val);
        Map<Integer, List<Integer>> graph = new HashMap<>();
        graph.put(node.val, new ArrayList<>());
        
        while (!queue.isEmpty()) {
            Node cur = queue.poll();
            for (Node nxt : cur.neighbors) {
                List<Integer> con = graph.getOrDefault(cur.val, new ArrayList<>());
                con.add(nxt.val);
                graph.put(cur.val, con);
                if (visited.add(nxt.val)) {
                    queue.add(nxt);
                }
            }
        }

        Map<Integer, Node> res = new HashMap<>();
        
        for (int key : visited) {
            res.put(key, new Node(key));
        }

        for (int cur : visited) {
            List<Node> tmp = res.get(cur).neighbors;
            for (int nxt : graph.get(cur)) {
                tmp.add(res.get(nxt));
            }
        }
        return res.get(node.val);
    }
}
