class Solution:
    def fourSum(self, nums: List[int], target: int) -> List[List[int]]:
        sortedElementList = sorted(nums)
        result = []
        self.nSumRecursive(sortedElementList, target, 4, result, [])
        return result
        
    def twoSum (self, listOfElements: list[int], target: int, resultList: list[list[int]], auxiliaryList: list[int], left: int) -> list[list[int]]:
        sortedList = sorted(listOfElements)
        lenght = len(sortedList)
        right = lenght - 1
        while (left < right):
            if (sortedList[left] + sortedList[right] < target): 
                left+=1
            elif (sortedList[left] + sortedList[right] > target):
                right -= 1
            else:
                resultList.append(auxiliaryList + [sortedList[left], sortedList[right]])
                left+=1
                while(left < right and sortedList[left] == sortedList[left - 1]): left += 1

        return

    def nSumRecursive (self, listOfElements: list[int], target: int, nSum: int, resultList: list[list[int]], auxiliaryList: list[int], left: int = 0) -> list[list[int]]:
        if (nSum == 2):
            self.twoSum(listOfElements, target, resultList, auxiliaryList, left)
        else:
            indexLimit = len(listOfElements) - nSum + 1
            for index in range(left, indexLimit):
                if (index == left or (listOfElements[index] != listOfElements[index - 1])):
                    self.nSumRecursive(listOfElements, target - listOfElements[index], nSum - 1, resultList, auxiliaryList + [listOfElements[index]], index + 1)
        return
        