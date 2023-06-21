array1 = [[1] * 10000] * 10000
array2 = [[1] * 10000] * 10000

for i in range(10000):
    for j in range(10000):
        array2[j][i] = array1[j][i]
