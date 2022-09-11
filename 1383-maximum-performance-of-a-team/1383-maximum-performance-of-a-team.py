class Solution:
    def maxPerformance(self, n: int, speed: List[int], efficiency: List[int], k: int) -> int:
        MOD = 10**9+7
        
        # sort by efficiency descending, speed descending
        # 역순으로 정렬한다.
        props = sorted(zip(efficiency, speed), key=lambda x: (-x[0], -x[1]))
        
        # 정답 변수
        global_ans = 0
        heap = []
        heap_speed_sum = 0
        eff_min = int(1e9)
        
        for eff, speed in props:
            # we first want to take k 
            if len(heap) < k:
                eff_min = min(eff_min, eff)
                heap_speed_sum += speed
                heapq.heappush(heap, (speed, eff))
                
            else:
                if speed > heap[0][0]:
                    popped_speed, popped_eff = heapq.heappop(heap)
                    heap_speed_sum -= popped_speed
                    
                    heap_speed_sum += speed
                    heapq.heappush(heap, (speed, eff))
                    eff_min = eff
                    
            global_ans = max(global_ans, eff_min * heap_speed_sum)
        
        
        return global_ans % MOD
