class Solution {
    public int[][] reconstructQueue(int[][] people) {
        Arrays.sort(people, (a1, a2) -> {
            if (a1[0] != a2[0]) {
                return a1[0] - a2[0];
            }
            return a2[1]-a1[1];
        });
        final List<int[]> res = new ArrayList<>();
        final int n = people.length;
        for (int i = n - 1; i >= 0; i--) {
            res.add(people[i][1], people[i]);
        }
        return res.toArray(new int[0][0]);
    }
}
