class Solution {
    public int[] deckRevealedIncreasing(final int[] deck) {
        final int n = deck.length;
        final int[] result = new int[n];
        Arrays.sort(deck);        
        final Queue<Integer> queue = new LinkedList<>();
        for (final int num : deck) {
            queue.add(num);
        }

        int index = 0;
        result[0] = queue.poll();
        while (!queue.isEmpty()) {
            final int num = queue.poll();
            int tmp = 0;
            while (tmp < 1) {
                if (result[index] == 0) {
                    tmp++;
                }
                index = (index + 1) % n;
            }
            while (result[index] != 0) {
                index = (index + 1) % n;
            }
            result[index] = num;
        }
        return result;
    }
}
