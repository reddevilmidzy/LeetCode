class Solution {
    public int findLeastNumOfUniqueInts(int[] arr, int k) {
        Map<Integer, Integer> cnt = new HashMap<>();
        
        for (int num : arr) {
            int val = cnt.getOrDefault(num, 0);
            cnt.put(num, val+1);
        }
        
        List<Integer> keys = new ArrayList<>(cnt.keySet());
        Collections.sort(keys, (v1, v2) -> (cnt.get(v1).compareTo(cnt.get(v2))));
        int rem = 0;
        
        for (int key : keys) {
            if (cnt.get(key) <= k) {
                k -= cnt.get(key);
                rem++;
            } else {
                break;
            }
        }
        return keys.size() - rem;
    }
}