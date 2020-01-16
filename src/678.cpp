class Solution {
public:
    bool checkValidString(string s) {
        int min = 0, max = 0;
        for (const auto c: s) {
            switch (c) {
                case '(':
                    min++; max++;
                    break;
                case ')':
                    min = ::max(0, min-1);
                    max--;
                    if (max < 0)
                        return false;
                    break;
                case '*':
                    min = ::max(0, min-1);
                    max ++;
                    break;
            }
        }
        return min <= 0 && 0 <= max;
    }
};