class RandomizedSet {

    private Map<Integer, Integer> index;
    private List<Integer> nums;
    private Random rand;
    
    public RandomizedSet() {
        this.index = new HashMap<>();
        this.nums = new ArrayList<>();
        this.rand = new Random();
    }
    
    public boolean insert(int val) {
        if (!index.containsKey(val)) {
            index.put(val, nums.size());
            nums.add(val);
            return true;
        }
        return false;
    }
    
    public boolean remove(int val) {
        if (index.containsKey(val)) {
            int remIdx = index.get(val);
            int lastVal = nums.get(nums.size()-1);
            nums.set(remIdx, lastVal);
            nums.remove(nums.size()-1);
            index.put(lastVal, remIdx);
            index.remove(val);
            
            return true;
        }
        return false;
    }
    
    public int getRandom() {
        int randomIndex = rand.nextInt(nums.size());
        return nums.get(randomIndex);
    }
}
