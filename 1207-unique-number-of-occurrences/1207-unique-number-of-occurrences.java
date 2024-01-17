class Solution {
    public boolean uniqueOccurrences(int[] arr) {
        Map<Integer, Integer> cnt = generateCounter(arr);
        return isUnique(new ArrayList<>(cnt.values()));
    }
    
    private Map<Integer, Integer> generateCounter(int[] arr) {
        Map<Integer, Integer> res = new HashMap<>();
        for (int num : arr) {
            res.put(num, res.getOrDefault(num, 0) + 1);
        }
        return res;
    }
    
    private boolean isUnique(List<Integer> arr) {
        return arr.size() == Set.copyOf(arr).size();
    }
}