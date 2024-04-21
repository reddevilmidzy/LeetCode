class Pair {
    final int timestamp;
    final String value;

    public Pair(final int timestamp, final String value) {
        this.timestamp = timestamp;
        this.value = value;
    }
}

class TimeMap {

    Map<String, List<Pair>> store;

    public TimeMap() {
        this.store = new HashMap<>();
    }
    
    public void set(final String key, final String value, final int timestamp) {
        if (!store.containsKey(key)) {
            store.put(key, new ArrayList<>());
        }
        final Pair pair = new Pair(timestamp, value);
        store.get(key).add(pair);
    }
    
    public String get(final String key, final int timestamp) {
        if (!store.containsKey(key)) {
            return "";
        }
        final int idx = findIndex(store.get(key), timestamp);

        if (idx == -1) {
            return "";
        }
        return store.get(key).get(idx).value;
    }

    private int findIndex(final List<Pair> pairs, final int timestamp) {
        int beg = 0;
        int end = pairs.size() - 1;

        while (beg <= end) {
            final int mid = beg + (end - beg) / 2;

            if (pairs.get(mid).timestamp == timestamp) {
                return mid;
            }
            if (pairs.get(mid).timestamp < timestamp) {
                beg = mid + 1;
            } else {
                end = mid - 1;
            }
        }
        return end;
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * TimeMap obj = new TimeMap();
 * obj.set(key,value,timestamp);
 * String param_2 = obj.get(key,timestamp);
 */
 