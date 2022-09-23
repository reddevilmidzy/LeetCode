class Solution:
    def concatenatedBinary(self, n: int) -> int:
        binary = ''
        for i in range(1, n+1):
            binary += str(bin(i%(10**9+7)))[2:]
#            print(str(bin(i))[2:])
        #binary = str('0b'+binary)
        # print(int(binary, 2))
        # print(binary)
        return int(binary, 2)%(10**9+7)
        