class Solution {
    public int[][] kClosest(int[][] points, int k) {
        int n = points.length;
        List<Pos> pos = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            int x = points[i][0];
            int y = points[i][1];
            pos.add(new Pos(x, y));
        }
        
        Collections.sort(pos);
        int[][] res = new int[k][2];
        
        for (int i = 0; i < k; i++) {
            Pos cur = pos.get(i);
            res[i][0] = cur.x;
            res[i][1] = cur.y;
        }
        return res;
    }
    
    static class Pos implements Comparable<Pos> {
        private int x;
        private int y;
        
        public Pos(int x, int y) {
            this.x = x;
            this.y = y;
        }
        
        @Override
        public int compareTo(Pos target) {
            long a = x*x + y*y;
            long b = target.x * target.x + target.y * target.y;
            
            if (a > b) {
                return 1;
            } else if (a < b) {
                return -1;
            }
            return 0;
        }
    
    }
}