/*
 * @lc app=leetcode.cn id=146 lang=cpp
 *
 * [146] LRU缓存机制
 */

// @lc code=start

#include <cassert>
#include <iostream>
#include <unordered_map>
#include <vector>

struct ValueNode {
    int key;
    int value;
    ValueNode *left, *right;
};

void print(ValueNode *head, ValueNode *tail) {
    auto ptr = head->right;
    std::cout << "(head) <-> ";
    while (ptr != tail) {
        std::cout << "(" << ptr->key << "," << ptr->value << ")"
                  << " <-> ";
        ptr = ptr->right;
    }
    std::cout << "(tail)" << std::endl;
}

class LRUCache {
    const int _capacity;
    int size;
    ValueNode *head, *tail;
    std::vector<ValueNode> node_pool;
    int node_pool_used;
    std::unordered_map<int, ValueNode *> hash;

    ValueNode *new_node(int key = 0, int value = 0) {
        assert(node_pool_used <= (_capacity + 2));
        auto ptr = &node_pool[node_pool_used++];
        ptr->key = key;
        ptr->value = value;
        ptr->left = nullptr;
        ptr->right = nullptr;
        return ptr;
    }
    void bind(ValueNode *left, ValueNode *right) {
        left->right = right;
        right->left = left;
    }

    void move_to_head(ValueNode *ptr) {
        if (ptr->left == head) {
            // pass
        } else {
            // strip
            auto left = ptr->left;
            auto right = ptr->right;
            bind(left, right);
            // put to head
            add_to_head(ptr);
        }
    }

    void add_to_head(ValueNode *ptr) {
        auto second = head->right;
        bind(head, ptr);
        bind(ptr, second);
    }

   public:
    LRUCache(int capacity)
        : _capacity(capacity),
          size(0),
          node_pool(capacity + 2),
          node_pool_used(0),
          hash() {
        head = new_node();
        tail = new_node();
        bind(head, tail);
    }

    int get(int key) {
        auto result = hash.find(key);
        if (result == hash.end()) {
            return -1;
        }
        auto ptr = result->second;
        // update LRU
        move_to_head(ptr);

        return ptr->value;
    }

    void put(int key, int value) {
        auto iter = hash.find(key);
        if (iter != hash.end()) {
            auto ptr = iter->second;
            ptr->value = value;
            move_to_head(ptr);
            return;
        }

        ValueNode *ptr = nullptr;
        // LRU
        if (size == _capacity) {
            // remove the last one from linked list
            ptr = tail->left;
            bind(ptr->left, tail);
            // drop key from hashmap
            auto old_key = ptr->key;
            hash.erase(old_key);
            // set ptr as key,value and put to head
            ptr->key = key;
            ptr->value = value;
            add_to_head(ptr);
        } else {
            // insert to linked list
            ptr = new_node(key, value);
            add_to_head(ptr);
            size++;
        }
        // insert new
        hash.insert(std::make_pair(key, ptr));
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */
// @lc code=end

int main() {
#define g(k) printf("get key %d -> %d\n", k, cache.get(k))
#define s(k, v)                     \
    printf("set %d -> %d\n", k, v); \
    cache.put(k, v)
    // auto cache = LRUCache(2);

    // cache.put(1, 1);
    // cache.put(2, 2);

    // assert(cache.get(1) == 1);   // 返回  1
    // cache.put(3, 3);             // 该操作会使得密钥 2 作废
    // assert(cache.get(2) == -1);  // 返回 -1 (未找到)
    // cache.put(4, 4);             // 该操作会使得密钥 1 作废
    // assert(cache.get(1) == -1);  // 返回 -1 (未找到)
    // assert(cache.get(3) == 3);   // 返回  3
    // assert(cache.get(4) == 4);   // 返回  4

    //
    auto cache = LRUCache(3);
    s(1, 1);
    s(2, 2);
    s(3, 3);
    s(4, 4);
    g(4);
    g(3);
    g(2);
    g(1);
    s(5, 5);
    g(1);
    g(2);
    g(3);
    g(4);
    g(5);
}
