class Solution:
    def totalFruit(self, fruits: List[int]) -> int:
        basket = defaultdict(int)
        i,j = 0,0
        res = [0]
        for i in range(len(fruits)):
            # dict에 과일 추가
            basket[fruits[i]] += 1
            # 종류가 2개 이상이라면 비우는 작업
            while len(basket) > 2:
                basket[fruits[j]] -= 1
                # 과일이 0개라면 dict에서 제거
                if basket[fruits[j]] == 0:
                    del basket[fruits[j]]
                j += 1
            res.append(i - j + 1)
        return max(res)