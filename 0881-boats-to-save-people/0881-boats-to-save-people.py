class Solution:
    def numRescueBoats(self, people: List[int], limit: int) -> int:
        n = len(people)
        people.sort()
        one, two = 0, n-1
        ans = 0
        # [3,3,4,5]
        # [1,2,2,3]
        while one <= two:
            tmp = people[one] + people[two]
            if tmp <= limit:
                ans += 1
                one += 1
                two -= 1
            elif tmp > limit:
                ans += 1
                two -= 1
        return ans