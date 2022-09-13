class Solution:
    def validUtf8(self, data: List[int]) -> bool:
        unicode=[]
        # 정답 배열
        for i in range(len(data)):
            # 이진수로 저장함 앞에 식별자는 제외
            x=bin(data[i]).replace("0b", "")
            if len(x)<8:
                x='0'*(8-len(x))+x
            unicode.append(x)
            
        curr=None
        cont=0
        for i in range(len(unicode)):
            if cont>0:
                if unicode[i][:2]!='10':
                    return False
                cont-=1
            elif cont==0 and unicode[i][:2]=='10':
                return False
            else:
                for j in range(5):
                    if unicode[i][j]=='0':
                        if j==0:
                            curr=1
                        else:
                            curr=j
                            cont=j-1
                        break
                else:
                    print("ok2")
                    return False
        if cont>0:
            return False
        return True