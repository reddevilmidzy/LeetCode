class Solution {
    public List<List<String>> accountsMerge(final List<List<String>> accounts) {
        final Map<String, Set<String>> graph = new HashMap<>();
        final Map<String, String> name = new HashMap<>();
        
        for (final List<String> account : accounts) {
            final String userName = account.get(0);
            for (int i = 1; i < account.size(); i++) {
                if (!graph.containsKey(account.get(i))) {
                    graph.put(account.get(i), new HashSet<>());
                }
                name.put(account.get(i), userName);
                
                if (i == 1) continue;
                graph.get(account.get(i)).add(account.get(i - 1));
                graph.get(account.get(i - 1)).add(account.get(i));
            }
        }
        
        final Set<String> visited = new HashSet<>();
        final List<List<String>> res = new LinkedList<>();
        
        for (final String email : name.keySet()) {
            final List<String> list = new LinkedList<>();
            if (visited.add(email)) {
                dfs(graph, email, visited, list);
                Collections.sort(list);
                list.add(0, name.get(email));
                res.add(list);
            }
        }
        
        return res;
    }
    
    public void dfs(final Map<String, Set<String>> graph, final String email, final  Set<String> visited, final List<String> list) {
        list.add(email);
        for (final String next : graph.get(email)) {
            if (visited.add(next)) {
                dfs(graph, next, visited, list);
            }
        }
    }
}