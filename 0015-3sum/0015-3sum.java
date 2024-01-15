class Solution {
    public List<List<Integer>> threeSum(int[] nums) {
        List<Integer> neg = new ArrayList<>();
        List<Integer> pos = new ArrayList<>();
        int zero = 0;
        
        for (int num : nums) {
            if (num == 0) {
                zero++;
            } else if (num > 0) {
                pos.add(num);
            } else if (num < 0) {
                neg.add(num);
            }
        }
        
        Set<List<Integer>> res = new HashSet<>();
        Set<Integer> negSet = Set.copyOf(neg);
        Set<Integer> posSet = Set.copyOf(pos);
        if (zero > 0) {
            if (zero >= 3) {
                res.add(List.of(0,0,0));
            }

            for (int num : posSet) {
                if (negSet.contains(-num)) {
                    res.add(List.of(-num, 0, num));
                }
            }
        }
                
        for (int i = 0; i < neg.size(); i++) {
            for (int j = i + 1; j < neg.size(); j++) {
                if (posSet.contains(-neg.get(i)-neg.get(j))) {
                    res.add(List.of(Math.min(neg.get(i), neg.get(j)), Math.max(neg.get(i), neg.get(j)), -neg.get(i)-neg.get(j)));
                }
            }
        }
        
        for (int i = 0; i < pos.size(); i++) {
            for (int j = i + 1; j < pos.size(); j++) {
                if (negSet.contains(-pos.get(i)-pos.get(j))) {
                    res.add(List.of(-pos.get(i)-pos.get(j), Math.min(pos.get(i), pos.get(j)), Math.max(pos.get(i), pos.get(j))));
                }
            }
        }
        
        return List.copyOf(res);
    }
}