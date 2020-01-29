flipped = lambda i: int(f'{i:08b}'[::-1], base=2)

for i in range(16):
    print(', '.join(f'{flipped(i):3d}' for i in range(i*16, (i+1)*16)), end=',\n')
