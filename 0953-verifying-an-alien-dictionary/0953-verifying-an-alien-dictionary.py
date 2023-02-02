class Solution:
    def order_num(self, word: str, order: str) -> List[int]:
        res = []
        for i in word:
            res.append(order.index(i))
        return res

    
    def isAlienSorted(self, words: List[str], order: str) -> bool:
        n = len(order)
        sorted_words = []
        alphabet = []
        
        for word in words:
            alphabet.append(self.order_num(word, order))
        
        sorted_words = sorted(alphabet)
        return sorted_words == alphabet