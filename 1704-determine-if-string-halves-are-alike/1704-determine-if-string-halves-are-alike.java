class Solution {
    public boolean halvesAreAlike(String s) {
        int n = s.length();
        int a = 0;
        int b = 0;        
        Set<Character> vowels = Set.of('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U');
        
        for (int i = 0; i < n; i++) {
            if (vowels.contains(s.charAt(i))) {
                if (i < n/2) {
                    a++;
                } else {
                    b++;
                }
            }
        }

        return a==b;
    }
}