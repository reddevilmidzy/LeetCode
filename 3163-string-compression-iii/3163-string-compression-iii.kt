class Solution {
    fun compressedString(word: String): String = buildString {
        var i = 0
        while (i < word.length) {
            var j = i
            while (j < minOf(i + 9, word.length) && word[i] == word[j]) j++
            append("${j - i}${word[i]}")
            i = j
        }
    }
}
