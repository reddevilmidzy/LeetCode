class MinStack {

    private final Stack<int[]> stack;
    private int minValue = Integer.MAX_VALUE;

    public MinStack() {
        this.stack = new Stack<>();
    }

    public void push(int val) {
        minValue = Math.min(minValue, val);
        stack.push(new int[] { val, minValue });
    }

    public void pop() {
        stack.pop();
        if (!stack.empty()) {
            minValue = stack.peek()[1];
            return;
        }
        minValue = Integer.MAX_VALUE;
    }

    public int top() {
        return stack.peek()[0];
    }

    public int getMin() {
        return stack.peek()[1];
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack obj = new MinStack();
 * obj.push(val);
 * obj.pop();
 * int param_3 = obj.top();
 * int param_4 = obj.getMin();
 */