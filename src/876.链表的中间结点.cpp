/*
 * @lc app=leetcode.cn id=876 lang=cpp
 *
 * [876] 链表的中间结点
 */

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};
// @lc code=start

class Solution {
   public:
    ListNode* middleNode(ListNode* head) {
        ListNode *fast = head, *slow = head;
        while (true) {
            fast = fast->next;
            if (fast == nullptr) {
                return slow;
            }
            slow = slow->next;
            //
            fast = fast->next;
            if (fast == nullptr) {
                return slow;
            }
        }
    }
};
// @lc code=end
