class Solution {
    public List<List<Integer>> findWinners(int[][] matches) {
        Set<Integer> winners = new HashSet<>();
        Map<Integer, Integer> losers = new HashMap<>();
        for (int[] match : matches) {
            int win = match[0];
            int lose = match[1];
            winners.add(win);
            losers.put(lose, losers.getOrDefault(lose, 0) + 1);
        }
        
        List<Integer> notLost = new ArrayList<>();
        for (int val : winners) {
            if (!losers.containsKey(val)) {
                notLost.add(val);
            }
        }
        Collections.sort(notLost);
        
        List<Integer> oneLost = new ArrayList<>();
        for (int key : losers.keySet()) {
            if (losers.get(key) == 1) {
                oneLost.add(key);
            }
        }
        Collections.sort(oneLost);
        
        return List.of(notLost, oneLost);
    }
}