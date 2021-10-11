

# x = 1, 2
def digit(x) -> str:
    return ['Zero', 'One', 'Two', 'Three', 'Four', 'Five', 'Six', 'Seven', 'Eight', 'Nine', 'Ten'][x]


# x = 20, 24
def xx_ty(x) -> str:
    return ['WTF', 'WTF', 'Twenty', 'Thirty', 'Forty', 'Fifty', 'Sixty', 'Seventy', 'Eighty', 'Ninety'][x // 10]


# x = 12, 18
def xx_teen(x) -> str:
    return [
        'Ten', 'Eleven', 'Twelve', 'Thirteen', 'Fourteen', 'Fifteen', 'Sixteen', 'Seventeen', 'Eighteen', 'Nineteen'
    ][x - 10]


# x = 102, 908, 0
def under_thousand(x) -> str:
    if x == 0:
        return []

    s = []
    if x // 100 > 0:
        s.extend([digit(x // 100), 'Hundred'])

    x %= 100
    if x >= 20:
        s.append(xx_ty(x))
        if y := x % 10:
            s.append(digit(y))
    elif x > 10:
        s.append(xx_teen(x))
    elif x == 0:
        pass
    else:  # 0 < x <= 10
        s.append(digit(x))

    return ' '.join(s)


class Solution:
    def numberToWords(self, num: int) -> str:
        if num == 0:
            return 'Zero'
        # assume num= 1,023,000,456

        # split by 1000
        group = []
        while num:
            group.append(num % 1000)
            num //= 1000
        # reversed
        # 456, 000, 023, 1

        xxllions = [
            None,
            'Thousand',
            'Million',
            'Billion',
            'Trillion',
        ]

        ans = []
        for xxllion, num in zip(xxllions, group):
            if num:
                if xxllion:
                    ans.append(xxllion)
                ans.append(under_thousand(num))

        return ' '.join(ans[::-1])


if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().numberToWords(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        0,
        'Zero'
    )
    test(
        100,
        'One Hundred'
    )
    test(
        123,
        "One Hundred Twenty Three"
    )
    test(12345,
         "Twelve Thousand Three Hundred Forty Five")
    test(
        1234567,
        "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
    )
    test(
        1234567891,
        "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
    )
