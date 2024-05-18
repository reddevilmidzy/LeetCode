class FreqStack {
    
    private final Map<Integer, Stack<Integer>> stk;
    private final Map<Integer, Integer> cnt;
    private int maxVal;

    public FreqStack() {
        this.stk = new HashMap();
        this.cnt = new HashMap();
        this.maxVal = 0;
    }
    
    public void push(final int val) {
        final int fre = cnt.getOrDefault(val, 0) + 1;
        maxVal = Math.max(maxVal, fre);
        cnt.put(val, fre);

        stk.putIfAbsent(fre, new Stack<>());
        stk.get(fre).add(val);
    }
    
    public int pop() {
        final Stack<Integer> s = stk.get(maxVal);
        final int val = s.pop();

        if (s.isEmpty()) {
            stk.remove(maxVal--);
        }

        final int fre = cnt.get(val);

        cnt.put(val, fre - 1);
        return val;
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * FreqStack obj = new FreqStack();
 * obj.push(val);
 * int param_2 = obj.pop();
 */
