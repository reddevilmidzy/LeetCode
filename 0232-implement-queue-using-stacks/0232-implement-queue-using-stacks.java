class MyQueue {

    private Stack<Integer> one;
    private Stack<Integer> two;
    
    public MyQueue() {
        this.one = new Stack<>();
        this.two = new Stack<>();
    }
    
    public void push(int x) {
        one.push(x);
    }
    
    public int pop() {
        while (one.size() != 1) {
            two.push(one.pop());
        }
        int res = one.pop();
        while (!two.isEmpty()) {
            one.push(two.pop());
        }
        return res;
    }
    
    public int peek() {
        // System.out.println(one);
        while (one.size() != 1) {
            two.push(one.pop());
        }
        // System.out.println(one);
        int res = one.pop();
        one.push(res);
        while (!two.isEmpty()) {
            one.push(two.pop());
        }
        return res;
    }
    
    public boolean empty() {
        return one.isEmpty();
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * MyQueue obj = new MyQueue();
 * obj.push(x);
 * int param_2 = obj.pop();
 * int param_3 = obj.peek();
 * boolean param_4 = obj.empty();
 */