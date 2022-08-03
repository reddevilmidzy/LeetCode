class MyCalendar:

    def __init__(self):
        self.calendar=[]

    def book(self, start: int, end: int) -> bool:
        calendar = self.calendar
        
        # 처음은 무조건 북할수 있음
        if len(calendar)==0:        
            calendar.append(start)
            calendar.append(end)
            return True
        
        i = bisect.bisect_right(self.calendar, start)
        if i%2: # i가 홀수라면 중간에 끼여있는 스케줄
            return False
        
        j = bisect.bisect_left(self.calendar, end)
        if i != j:
            return False
        
        self.calendar[i:i] = [start, end]
        return True
        
        


# Your MyCalendar object will be instantiated and called as such:
# obj = MyCalendar()
# param_1 = obj.book(start,end)