print(','.join(
    str(sum(int(x) for x in f'{i:b}'))
    for i in range(0xFF + 1)
))
