class Solution:
    def isPowerOfThree(self, n: int) -> bool:
        power_of_three = [3**i for i in range(20)]
        if n in power_of_three:
            return True
        else:
            return False