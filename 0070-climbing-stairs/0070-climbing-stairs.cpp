class Solution {
public:
    int climbStairs(int n) {
        int one = 0;
        int two = 1;
        int three = 0;
        
        for (int i = 0; i < n; i++) {
            three = one + two;
            one = two;
            two = three;
        }
        
        return two;
    }
};