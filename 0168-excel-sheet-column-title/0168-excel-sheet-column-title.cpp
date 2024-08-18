class Solution {
public:
    string convertToTitle(int columnNumber) {
        string res = "";
        while (columnNumber)
        {
            columnNumber--;
            int digit = columnNumber % 26 + 65;
            columnNumber /= 26;
            res = (char) (digit) + res;
        }
        return res;
    }
};
