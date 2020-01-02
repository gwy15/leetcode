/*
 * @lc app=leetcode id=160 lang=cpp
 *
 * [160] Intersection of Two Linked Lists
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 */
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
   public:
    int getLength(ListNode *head) {
        int len = 0;
        while (head != nullptr) {
            head = head->next;
            len++;
        }
        return len;
    }
    ListNode* move(ListNode* ptr, int len) {
        while (len--) {
            ptr = ptr->next;
        }
        return ptr;
    }
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        int lenA = getLength(headA);
        int lenB = getLength(headB);
        if (lenA > lenB) {
            headA = move(headA, lenA-lenB);
        } else {
            headB = move(headB, lenB-lenA);
        }
        while (headA && headB) {
            if (headA == headB) {
                return headA;
            }
            headA = headA->next;
            headB = headB->next;
        }
        return nullptr;
    }
};
// @lc code=end
