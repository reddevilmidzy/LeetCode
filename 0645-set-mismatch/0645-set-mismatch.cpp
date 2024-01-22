class Solution {
public:
    vector<int> findErrorNums(vector<int>& nums) {
        int n = nums.size();
        int xorAll = 0;
        int xorArray = 0;
        
        
        // 1 ~ n xor
        for (int i = 1; i <= n; i++) {
            xorAll ^= i;
        }
        
        // given nums xor
        for (int num : nums) {
            xorArray ^= num;
        }
        
        int xorResult = xorArray ^ xorAll;
        
        // 최상위 비트
        int rightmostSetBit = xorResult & -xorResult;
        
        int xorSet = 0;
        int xorNotSet = 0;
        
        // 추가 
        for (int i = 1; i <= n; i++) {
            if (i & rightmostSetBit) {
                xorSet ^= i;
            } else {
                xorNotSet ^= i;
            }
        }
        
        //제거
        for (int num : nums) {
            if (num & rightmostSetBit) {
                xorSet ^= num;
            } else {
                xorNotSet ^= num;
            }
        }
        
        for (int num : nums) {
            // 있다면 중복된 녀석이 xorSet
            if (num == xorSet) {
                return {xorSet, xorNotSet};
            }
        }
        // 없다면 xorNotSet 이 중복된 녀석
        return {xorNotSet, xorSet};
    }
};