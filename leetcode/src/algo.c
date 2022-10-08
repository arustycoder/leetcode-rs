//
// Created by simon on 2022/9/30.
//
// Definition for single-linked list
#include<stdlib.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

// =142=
struct ListNode *detectCycle(struct ListNode *head) {
    struct ListNode *slow, *fast;
    slow = fast = head;
    while (fast != NULL && fast->next != NULL) {
        slow = slow->next;
        fast = fast->next->next;
        if (fast == slow) {
            struct ListNode *tmp = head;
            while (tmp != slow) {
                tmp = tmp->next;
                slow = slow->next;
            }
            return tmp;
        }
    }
    return NULL;
}

/**
 * Definition for Node in problem 589
 */
struct Node {
    int val;
    int numChildren;
    struct Node** children;
};

void helper(const struct Node* root, int* res, int* pos) {
    if (root == NULL) {
        return;
    }

    res[(*pos)++] = root->val;
    for (int i=0; i<root->numChildren; i++) {
        helper(root->children[i], res, pos);
    }
}

int* preorder(struct Node* root, int* returnSize) {
    //1. 减少内存分配，假设一个最大值，题目中已经给出 [0,10^4]
    //2. 递归
    //3. 必须使用栈记录现在遍历的位置，避免陷入思维误区，要注意套路
#define MAX_NODE_SIZE 10000

    int* res = (int*)malloc(MAX_NODE_SIZE * sizeof(int));
    int pos = 0;
    helper(root, res, &pos);
    *returnSize = pos;
    return res;
}