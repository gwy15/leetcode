/*
 * @lc app=leetcode.cn id=25 lang=cpp
 *
 * [25] K 个一组翻转链表
 */

#include <iostream>
#include <vector>

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

std::ostream& operator<<(std::ostream& os, ListNode* ptr) {
    while (ptr != nullptr) {
        os << ptr->val << " -> ";
        ptr = ptr->next;
    }
    return os;
}

// @lc code=start
class Solution {
   public:
    ListNode* reverseKGroup(ListNode* head, int k) {
        // if length < k, return head
        ListNode* ptr = head;
        for (int i = 0; i < k; i++) {
            if (ptr == nullptr) {
                return head;
            }
            ptr = ptr->next;
        }
        // flip k nodes
        ListNode* prev = nullptr;
        ptr = head;
        for (int i = 0; i < k; i++) {
            ListNode* next_ptr = ptr->next;
            ptr->next = prev;
            prev = ptr;
            ptr = next_ptr;
        }
        // flip next k nodes if needed
        ptr = reverseKGroup(ptr, k);
        // now link leftmost node with current ptr
        head->next = ptr;
        // return new head
        return prev;
    }
};
// @lc code=end

ListNode* list2node(std::vector<int> v) {
    auto head = new ListNode(0);
    auto ptr = head;
    for (auto n : v) {
        ptr->next = new ListNode(n);
        ptr = ptr->next;
    }
    return head->next;
}

int main() {
    auto s = new Solution();
    auto nodes = list2node({1, 2, 3, 4, 5});
    std::cout << nodes << std::endl;
    nodes = s->reverseKGroup(nodes, 2);
    std::cout << nodes << std::endl;
}