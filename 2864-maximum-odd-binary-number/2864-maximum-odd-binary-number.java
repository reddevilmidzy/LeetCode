class Solution {
    public String maximumOddBinaryNumber(String s) {
        int zeroCount = count(s, '0');
        int oneCount = count(s, '1');

        return generateMaximumOddBinaryNumber(zeroCount, oneCount);
    }

    private int count(String word, char target) {
        return (int) word.chars()
                .filter(c -> c == target)
                .count();
    }

    private String generateMaximumOddBinaryNumber(int zeroCount, int oneCount) {
        return "1".repeat(oneCount - 1) + "0".repeat(zeroCount) + "1";
    }
}