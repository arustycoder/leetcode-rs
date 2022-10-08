//
// Created by simon on 2022/9/30.
//
// Definition for single-linked list
struct ListNode {
    int val;
    struct ListNode *next;
};

#include<stdlib.h>

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