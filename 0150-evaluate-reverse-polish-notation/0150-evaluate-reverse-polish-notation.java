class Solution {
    public int evalRPN(String[] tokens) {
        Set<String> ops = Set.of("+", "-", "*", "/");
        Stack<String> stk = new Stack<>();
        int res = Integer.parseInt(tokens[0]);
        
        for (String token : tokens) {
            if (!ops.contains(token)) {
                stk.push(token);
            } else {
                String op2 = stk.pop();
                String op1 = stk.pop();
                res = operate(op1, op2, token);
                stk.push(String.valueOf(res));
            }
        }
        
        return res;
    }
    
    private int operate(String operand1, String operand2, String operate) {
        int val1 = Integer.parseInt(operand1);
        int val2 = Integer.parseInt(operand2);
        
        if (operate.equals("+")) {
            return val1 + val2;
        }
        if (operate.equals("-")) {
            return val1 - val2;
        }
        if (operate.equals("*")) {
            return val1 * val2;
        }
        return val1 / val2;
    }
}
