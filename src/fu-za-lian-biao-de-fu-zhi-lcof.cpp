// Definition for a Node.
class Node {
   public:
    int val;
    Node* next;
    Node* random;

    Node(int _val, Node* next = nullptr, Node* random = nullptr) {
        val = _val;
        this->next = next;
        this->random = random;
    }
};
// @leetcode start
class Solution {
   public:
    Node* copyRandomList(Node* head) {
        if (head == nullptr) {
            return nullptr;
        }
        auto ptr = head;
        // copy value and insert to linked list
        while (ptr != nullptr) {
            auto next = ptr->next;
            // make a new copy and insert
            auto copy = new Node(ptr->val);
            // link start!
            ptr->next = copy;
            copy->next = next;
            // move ptr
            ptr = next;
        }
        // copy random ptr
        ptr = head;
        while (ptr != nullptr) {
            if (ptr->random != nullptr) {
                ptr->next->random = ptr->random->next;
            }
            ptr = ptr->next->next;
        }
        // move ptr out
        ptr = head;
        auto result = head->next;
        while (ptr != nullptr) {
            auto next = ptr->next;
            if (next != nullptr) ptr->next = ptr->next->next;
            ptr = next;
        }
        return result;
    }
};

int main() {
    auto head = new Node(7);
    auto n13 = new Node(13);
    auto n11 = new Node(11);
    head->next = n13;
    head->random = n11;
    n13->next = n11;
    n13->random = head;
    n11->random = head;
}
