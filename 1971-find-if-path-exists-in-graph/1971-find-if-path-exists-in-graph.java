class Solution {
    public boolean validPath(final int n, final int[][] edges, final int source, final int destination) {
        List<List<Integer>> graph = new ArrayList<>();

        for (int i = 0; i < n; i++) {
            graph.add(new ArrayList<>());
        }

        for (final int[] edge : edges) {
            final int x = edge[0];
            final int y = edge[1];
            graph.get(x).add(y);
            graph.get(y).add(x);
        }

        final Queue<Integer> queue = new LinkedList<>();
        queue.add(source);
        final boolean[] visited = new boolean[n];
        visited[source] = true;
        
        while (!queue.isEmpty()) {
            final int cur = queue.poll();
            if (cur == destination) {
                return true;
            }
            for (final int nxt : graph.get(cur)) {
                if (!visited[nxt]) {
                    queue.add(nxt);
                    visited[nxt] = true;
                }
            }
        }
        return false;
    }
}
