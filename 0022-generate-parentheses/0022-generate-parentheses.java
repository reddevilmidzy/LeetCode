class Solution {

    public List<String> generateParenthesis(final int n) {
        final List<String> res = new ArrayList<>();
        bt(n*2, "", 0, 0, res);
        return res;
    }

    private void bt(final int n, final String str, final int left, final int right, final List<String> res) {
        if (left + right == n) {
            res.add(str);
            return;
        }

        if (left==right) {
            bt(n, str + "(", left + 1, right, res);
        } else if (left < n / 2) {
            bt(n, str + "(", left + 1, right, res);
            if (left > right) {
                bt(n, str + ")", left, right + 1, res);
            }
        } else {
            bt(n, str + ")", left, right + 1, res);
        }
    }
}
