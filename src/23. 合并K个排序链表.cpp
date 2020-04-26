#include <vector>
#include <algorithm>
#include <iostream>

using namespace std;

struct ListNode
{
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};

bool operator<(ListNode lhs, ListNode rhs)
{
    return lhs.val > rhs.val;
}

class Solution
{
public:
    ListNode *mergeKLists(vector<ListNode *> &lists)
    {
        std::vector<ListNode> heap;
        for (auto node : lists)
        {
            if (node != nullptr)
            {
                heap.push_back(*node);
                std::push_heap(heap.begin(), heap.end());
            }
        }
        //
        ListNode start(0);
        ListNode *ptr = &start;
        while (heap.size() > 0)
        {
            std::pop_heap(heap.begin(), heap.end());
            auto node = heap.back();
            heap.pop_back();

            if (node.next != nullptr)
            {
                heap.push_back(*node.next);
                std::push_heap(heap.begin(), heap.end());
            }

            auto val = node.val;
            ptr->next = new ListNode(val);
            ptr = ptr->next;
        }
        return start.next;
    }
};

int main()
{
    vector<ListNode *> lists = {
        new ListNode(1),
        new ListNode(2),
        new ListNode(3)};
    Solution().mergeKLists(lists);
}
