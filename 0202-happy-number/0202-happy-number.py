class Solution:
    def isHappy(self, n: int) -> bool:
        v = {n}
        cur = n
        cnt = 0
        flag = True
        while True:
            cur = self.calculate(str(cur))
            cnt += 1
            print('cur', cur)
            if cur == 1: return True
            flag = flag & ('1' not in str(cur))

            if cur in v: return flag


            if cnt > 100: break

        return False

    def calculate(self, n: str) -> int:
        return sum([int(i)**2 for i in n])
