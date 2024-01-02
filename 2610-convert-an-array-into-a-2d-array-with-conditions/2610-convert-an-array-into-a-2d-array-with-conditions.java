class Solution {
    
    public List<List<Integer>> findMatrix(int[] nums) {
        Map<Integer, Integer> res = new HashMap<>();
        List<List<Integer>> ans = new ArrayList<>();
        for (int num : nums) {
            if (res.containsKey(num)) {
                res.put(num, res.get(num) + 1);
                continue;
            }
            res.put(num, 1);
        }

        while (!res.isEmpty()) {
            List<Integer> tmp = new ArrayList<>();
            List<Integer> removed = new ArrayList<>();

            for (Integer key : res.keySet()) {
                int val = res.get(key);
                tmp.add(key);
                if (val == 1) {
                    removed.add(key);
                    continue;
                }
                res.put(key, val - 1);
            }
            for (Integer rem : removed) {
                res.remove(rem);
            }
            ans.add(tmp);
        }
        return ans;
    }
}