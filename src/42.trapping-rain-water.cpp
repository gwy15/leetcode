/*
 * @lc app=leetcode id=42 lang=cpp
 *
 * [42] Trapping Rain Water
 */
#include <stack>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
   public:
    int trap(vector<int>& height) {
        if (height.size() <= 1) {
            return 0;
        }
        int sum = 0;
        int left = 0, right = height.size() - 1;
        int maxLeft = height[left], maxRight = height[right];
        while (left + 1 < right) {
            if (maxLeft < maxRight) {
                left++;  // left move right
                int curHeight = height[left];
                if (curHeight > maxLeft) {
                    maxLeft = curHeight;  // grow higher, but no rain
                } else {
                    sum += maxLeft - curHeight;  // trap rain water
                }
            } else {  // similar
                right--;
                int curHeight = height[right];
                if (curHeight > maxRight) {
                    maxRight = curHeight;
                } else {
                    sum += maxRight - curHeight;
                }
            }
        }
        return sum;
    }
};
