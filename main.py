n = int(input())
a_list = input().split()
q = int(input())

result = []

for i in range(q) :
    q_list = input().split()
    if int(q_list[0]) == 1:
        # 変換
        a_list[int(q_list[1])-1] = int(q_list[2])
    else:
        result.append(a_list[int(q_list[1])-1])

for i in result:
    print(i)
