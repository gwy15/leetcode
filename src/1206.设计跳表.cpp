/*
 * @lc app=leetcode.cn id=1206 lang=cpp
 *
 * [1206] 设计跳表
 */

#include <vector>
using std::vector;
// @lc code=start
#include <cstdlib>

const int N = 16;

struct SkiplistNode {
    int val;
    int count;
    vector<SkiplistNode*> next;

    SkiplistNode(int v) : val(v), count(1), next() {}
};

class Skiplist {
    SkiplistNode head;

    static int random_level() {
        int level = 1;
        while (level < N && std::rand() < 0.25 * RAND_MAX) {
            level += 1;
        }
        return level;
    }

   public:
    Skiplist() : head(-1) { head.next = vector<SkiplistNode*>(N, nullptr); }

    bool search(int target) {
        SkiplistNode* ptr = &head;
        for (int level = N - 1; level >= 0; level--) {
            while (ptr->next[level] != nullptr &&
                   ptr->next[level]->val < target) {
                ptr = ptr->next[level];
            }
        }
        return ptr->next[0] != nullptr && ptr->next[0]->val == target;
    }

    void add(int num) {
        SkiplistNode* ptr = &head;
        vector<SkiplistNode*> stack(N, nullptr);
        for (int level = N - 1; level >= 0; level--) {
            while (ptr->next[level] != nullptr && ptr->next[level]->val < num) {
                ptr = ptr->next[level];
            }
            stack[level] = ptr;
        }
        if (ptr->next[0] != nullptr && ptr->next[0]->val == num) {
            ptr->next[0]->count += 1;
            return;
        }
        // make new node
        SkiplistNode* new_node = new SkiplistNode(num);
        int level = random_level();
        new_node->next = vector<SkiplistNode*>(level, nullptr);
        for (int i = 0; i < level; i++) {
            SkiplistNode* prev = stack[i];
            new_node->next[i] = prev->next[i];
            prev->next[i] = new_node;
        }
    }

    bool erase(int num) {
        SkiplistNode* ptr = &head;
        vector<SkiplistNode*> stack(N, nullptr);
        for (int level = N - 1; level >= 0; level--) {
            while (ptr->next[level] != nullptr && ptr->next[level]->val < num) {
                ptr = ptr->next[level];
            }
            stack[level] = ptr;
        }
        ptr = ptr->next[0];
        if (ptr == nullptr || ptr->val != num) {
            return false;
        }
        if (ptr->count > 1) {
            ptr->count -= 1;
            return true;
        }
        // remove ptr
        for (int i = 0; i < ptr->next.size(); i++) {
            stack[i]->next[i] = ptr->next[i];
        }
        delete ptr;
        return true;
    }
};

// @lc code=end
#include <cassert>
int main() {
    Skiplist* obj = new Skiplist();
    assert(obj->search(1) == false);
    obj->add(2);
    assert(obj->search(2));
    assert(obj->erase(3) == false);
    assert(obj->erase(2) == true);
}
